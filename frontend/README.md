# Stellar Guilds Frontend

A modern Next.js frontend for the Stellar Guilds decentralized platform.

## Features

- ✅ Next.js 14 with App Router
- ✅ TypeScript support
- ✅ Tailwind CSS with custom Stellar theme
- ✅ Responsive design (mobile + desktop)
- ✅ Zustand for state management
- ✅ Reusable UI component library
- ✅ Mock data system for development
- ✅ Clean folder structure

## Getting Started

1. Install dependencies:
```bash
npm install
```

2. Run the development server:
```bash
npm run dev
```

3. Open [http://localhost:3000](http://localhost:3000) in your browser

## Project Structure

```
src/
├── app/                 # Next.js App Router pages
│   ├── layout.tsx      # Root layout
│   ├── page.tsx        # Home page
│   └── globals.css     # Global styles
├── components/         # React components
│   ├── ui/            # Reusable UI components
│   ├── layout/        # Layout components
│   └── index.ts       # Component exports
├── lib/               # Utilities and helpers
│   ├── mocks/         # Mock data generators
│   └── utils.ts       # Utility functions
├── store/             # Zustand stores
├── types/             # TypeScript types
└── hooks/             # Custom React hooks
```

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Create production build
- `npm run start` - Start production server
- `npm run lint` - Run ESLint

## Tech Stack

- **Framework**: Next.js 14
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State Management**: Zustand
- **Icons**: Lucide React
- **Form Handling**: React Hook Form + Zod

## Design System

The UI uses a custom Stellar-themed design system with:

- **Primary Colors**: Deep space navy, cyan, and white
- **Secondary Colors**: Gold and silver for tier indicators
- **Typography**: System font stack for optimal performance
- **Components**: Consistent spacing, shadows, and transitions

## Development

To add new features:

1. Create components in the appropriate folders (`ui/`, `layout/`, or `features/`)
2. Use the existing TypeScript types in `types/ui.ts`
3. Leverage the mock data system for testing
4. Follow the established component patterns

## Learn More

- [Next.js Documentation](https://nextjs.org/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)