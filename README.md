# Dioxus Rerun RRD Viewer

A Rust fullstack application using Dioxus to display Rerun RRD files from AWS S3 presigned URLs.

## Features

- 🎯 **Dioxus Fullstack**: Modern Rust web framework for both frontend and backend
- 🔗 **AWS S3 Integration**: Support for presigned URLs to access RRD files
- 🖼️ **Rerun Viewer**: Embedded Rerun viewer via iframe
- 🧪 **Testing Setup**: Dummy RRD file for testing and development
- 🎨 **Modern UI**: Clean and responsive user interface

## Project Structure

```
dioxus_rerun/
├── frontend/          # Dioxus web frontend
│   ├── src/main.rs    # Frontend application
│   ├── public/        # Static assets
│   └── Cargo.toml     # Frontend dependencies
├── backend/           # Axum backend server
│   ├── src/main.rs    # Backend server
│   └── Cargo.toml     # Backend dependencies
├── build.sh          # Build script
└── Cargo.toml        # Workspace configuration
```

## Prerequisites

- Rust (latest stable version)
- Cargo
- Dioxus CLI: `cargo install dioxus-cli`

## Quick Start

1. **Clone and navigate to the project:**
   ```bash
   cd dioxus_rerun
   ```

2. **Build the project:**
   ```bash
   ./build.sh
   ```

3. **Run the backend server:**
   ```bash
   cargo run --bin backend
   ```

4. **Open your browser:**
   Navigate to `http://localhost:3000`

## Usage

### Testing with Dummy RRD

1. Click the "Load Dummy RRD" button to view a sample RRD file
2. The application will load a public RRD file from the Rerun examples

### Using Custom AWS S3 URLs

1. Enter your AWS S3 presigned URL in the input field
2. Click "Load Custom URL" to view your RRD file
3. The Rerun viewer will be embedded in an iframe

## API Endpoints

- `GET /health` - Health check endpoint
- `GET /api/dummy-rrd` - Get dummy RRD URL for testing
- `POST /api/rrd` - Process custom RRD URLs
- `GET /` - Main application page

## Development

### Frontend Development

```bash
cd frontend
dioxus serve
```

### Backend Development

```bash
cd backend
cargo run
```

## How It Works

1. **Frontend**: Dioxus web application that provides a UI for loading RRD files
2. **Backend**: Axum server that handles API requests and serves the frontend
3. **Rerun Integration**: Uses Rerun's web viewer via iframe: 
   ```html
   <iframe src="https://app.rerun.io/version/{RERUN_VERSION}/index.html?url={RRD_URL}"></iframe>
   ```

## Configuration

### Rerun Version

The Rerun viewer version can be configured in `frontend/src/main.rs`:

```rust
let rerun_version = "0.15.0"; // Change this to use different Rerun versions
```

### CORS Settings

CORS is configured in `backend/src/main.rs` to allow frontend-backend communication.

## Troubleshooting

### Common Issues

1. **Build fails**: Ensure you have the latest Rust toolchain
2. **CORS errors**: Check that the backend is running on the correct port
3. **RRD not loading**: Verify the presigned URL is valid and accessible

### Debug Mode

Run the backend with debug logging:

```bash
RUST_LOG=debug cargo run --bin backend
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License. 