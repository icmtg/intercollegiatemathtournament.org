# Intercollegiate Math Tournament - Frontend

The frontend application for the Intercollegiate Math Tournament website, built with React, TypeScript, and TanStack Router.

## Tech Stack

- **React** with TypeScript
- **TanStack Router** for file-based routing
- **Tailwind CSS** for styling
- **Vite** for development and build tooling
- **Axios** for API communication

## Getting Started

### Prerequisites

- Node.js (v18 or higher)
- npm or yarn

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

The development server will start on `http://localhost:5173` with hot module replacement.

### Build

```bash
npm run build
```

## Project Structure

```
src/
├── routes/           # File-based routes (TanStack Router)
│   ├── __root.tsx   # Root layout
│   ├── index.tsx    # Home page
│   ├── login.tsx    # Login page
│   └── register.tsx # Registration page
├── lib/             # Utility functions and configurations
├── main.tsx         # Application entry point
└── index.css        # Global styles
```

## API Integration

The frontend communicates with the backend through `/api` endpoints. During development, requests are proxied to `http://localhost:3000` via Vite's proxy configuration.

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build locally
- `npm run lint` - Run ESLint
