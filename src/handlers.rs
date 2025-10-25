use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::env;

use crate::db::DbPool;
use crate::models::{NewUrl, Url};
use crate::schema::urls;
use crate::utils::{generate_short_code, validate_url};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
    pub short_code: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn shorten_url(
    pool: web::Data<DbPool>,
    request: web::Json<ShortenRequest>,
) -> impl Responder {
    // Validate the URL
    let original_url = match validate_url(&request.url) {
        Ok(url) => url,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid URL: {}", err),
            });
        }
    };

    // Generate a unique short code
    let mut short_code = generate_short_code();
    let mut attempts = 0;
    const MAX_ATTEMPTS: i32 = 10;

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Database connection error".to_string(),
            });
        }
    };

    // Try to insert with a unique short code
    loop {
        let new_url = NewUrl {
            original_url: original_url.clone(),
            short_code: short_code.clone(),
        };

        let result = diesel::insert_into(urls::table)
            .values(&new_url)
            .get_result::<Url>(&mut conn)
            .await;

        match result {
            Ok(_) => break,
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => {
                attempts += 1;
                if attempts >= MAX_ATTEMPTS {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to generate unique short code".to_string(),
                    });
                }
                short_code = generate_short_code();
            }
            Err(_) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".to_string(),
                });
            }
        }
    }

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let short_url = format!("{}/{}", base_url, short_code);

    HttpResponse::Ok().json(ShortenResponse {
        short_url,
        short_code,
    })
}

pub async fn redirect(pool: web::Data<DbPool>, short_code: web::Path<String>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Database connection error");
        }
    };

    let result = urls::table
        .filter(urls::short_code.eq(short_code.as_str()))
        .first::<Url>(&mut conn)
        .await;

    match result {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url.original_url))
            .finish(),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Short URL not found")
        }
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Shorty - URL Shortener</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, #10b981 0%, #059669 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }

        .container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            padding: 40px;
            max-width: 600px;
            width: 100%;
        }

        h1 {
            color: #10b981;
            font-size: 2.5em;
            margin-bottom: 10px;
            text-align: center;
        }

        .subtitle {
            color: #666;
            text-align: center;
            margin-bottom: 30px;
            font-size: 1.1em;
        }

        .form-group {
            margin-bottom: 20px;
        }

        label {
            display: block;
            color: #333;
            font-weight: 600;
            margin-bottom: 8px;
            font-size: 0.95em;
        }

        input[type="url"] {
            width: 100%;
            padding: 15px;
            border: 2px solid #e0e0e0;
            border-radius: 10px;
            font-size: 1em;
            transition: all 0.3s ease;
            font-family: inherit;
        }

        input[type="url"]:focus {
            outline: none;
            border-color: #10b981;
            box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
        }

        button {
            width: 100%;
            padding: 15px;
            background: linear-gradient(135deg, #10b981 0%, #059669 100%);
            color: white;
            border: none;
            border-radius: 10px;
            font-size: 1.1em;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s ease, box-shadow 0.2s ease;
        }

        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(16, 185, 129, 0.3);
        }

        button:active {
            transform: translateY(0);
        }

        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }

        .result {
            margin-top: 25px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 10px;
            display: none;
        }

        .result.show {
            display: block;
            animation: slideIn 0.3s ease;
        }

        @keyframes slideIn {
            from {
                opacity: 0;
                transform: translateY(-10px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .result h3 {
            color: #10b981;
            margin-bottom: 10px;
            font-size: 1.2em;
        }

        .short-url {
            display: flex;
            gap: 10px;
            align-items: center;
        }

        .short-url input {
            flex: 1;
            padding: 12px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-size: 1em;
            background: white;
        }

        .copy-btn {
            padding: 12px 20px;
            background: #10b981;
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            font-weight: 600;
            white-space: nowrap;
            transition: background 0.2s ease;
        }

        .copy-btn:hover {
            background: #059669;
        }

        .copy-btn.copied {
            background: #34d399;
        }

        .error {
            margin-top: 15px;
            padding: 15px;
            background: #fee;
            border-left: 4px solid #dc3545;
            border-radius: 8px;
            color: #dc3545;
            display: none;
        }

        .error.show {
            display: block;
            animation: slideIn 0.3s ease;
        }

        @media (max-width: 600px) {
            .container {
                padding: 30px 20px;
            }

            h1 {
                font-size: 2em;
            }

            .short-url {
                flex-direction: column;
            }

            .copy-btn {
                width: 100%;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔗 Shorty</h1>
        <p class="subtitle">Shorten your URLs in seconds</p>

        <form id="shortenForm">
            <div class="form-group">
                <label for="url">Enter your long URL</label>
                <input
                    type="url"
                    id="url"
                    name="url"
                    placeholder="https://example.com/very/long/url/that/needs/shortening"
                    required
                    autocomplete="off"
                >
            </div>
            <button type="submit" id="submitBtn">Shorten URL</button>
        </form>

        <div class="result" id="result">
            <h3>Your shortened URL:</h3>
            <div class="short-url">
                <input type="text" id="shortUrl" readonly>
                <button class="copy-btn" id="copyBtn">Copy</button>
            </div>
        </div>

        <div class="error" id="error"></div>
    </div>

    <script>
        const form = document.getElementById('shortenForm');
        const urlInput = document.getElementById('url');
        const submitBtn = document.getElementById('submitBtn');
        const result = document.getElementById('result');
        const shortUrlInput = document.getElementById('shortUrl');
        const copyBtn = document.getElementById('copyBtn');
        const errorDiv = document.getElementById('error');

        form.addEventListener('submit', async (e) => {
            e.preventDefault();

            const url = urlInput.value.trim();
            if (!url) return;

            // Hide previous results/errors
            result.classList.remove('show');
            errorDiv.classList.remove('show');

            // Disable button during request
            submitBtn.disabled = true;
            submitBtn.textContent = 'Shortening...';

            try {
                const response = await fetch('/api/shorten', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ url }),
                });

                const data = await response.json();

                if (response.ok) {
                    shortUrlInput.value = data.short_url;
                    result.classList.add('show');
                    copyBtn.textContent = 'Copy';
                    copyBtn.classList.remove('copied');
                } else {
                    errorDiv.textContent = data.error || 'An error occurred';
                    errorDiv.classList.add('show');
                }
            } catch (error) {
                errorDiv.textContent = 'Network error. Please try again.';
                errorDiv.classList.add('show');
            } finally {
                submitBtn.disabled = false;
                submitBtn.textContent = 'Shorten URL';
            }
        });

        copyBtn.addEventListener('click', async () => {
            try {
                await navigator.clipboard.writeText(shortUrlInput.value);
                copyBtn.textContent = 'Copied!';
                copyBtn.classList.add('copied');

                setTimeout(() => {
                    copyBtn.textContent = 'Copy';
                    copyBtn.classList.remove('copied');
                }, 2000);
            } catch (error) {
                // Fallback for older browsers
                shortUrlInput.select();
                document.execCommand('copy');
                copyBtn.textContent = 'Copied!';
            }
        });

        // Clear error when user starts typing
        urlInput.addEventListener('input', () => {
            errorDiv.classList.remove('show');
        });
    </script>
</body>
</html>"#,
    )
}
