# Database Setup Checklist

## Pre-Setup Requirements

- [ ] Node.js >= 18 installed
- [ ] Docker & Docker Compose installed
- [ ] Git cloned successfully
- [ ] You are in the `/workspaces/Stellar-Guilds/backend` directory

## Installation Phase

- [ ] Run `npm install` (installs all dependencies including Prisma)
- [ ] Verify with `npm run db:generate` (should show ✔ Generated Prisma Client)
- [ ] Verify with `npm run build` (should complete with no errors)

## Configuration Phase

- [ ] Copy `.env.example` to `.env`
- [ ] Verify `.env` contains: `DATABASE_URL="postgresql://postgres:postgres@localhost:5432/stellar_guilds_dev"`
- [ ] Verify `.env` contains: `PORT=3000`
- [ ] Verify `.env` contains: `NODE_ENV=development`

## Database Setup Phase

- [ ] Start PostgreSQL: `docker-compose up -d`
- [ ] Wait 30 seconds for PostgreSQL to be ready
- [ ] Verify PostgreSQL running: `docker-compose ps` (should show postgres status as Up)
- [ ] Check PostgreSQL logs: `docker-compose logs postgres` (look for "ready to accept connections")

## Migration Phase

- [ ] Run migration: `npm run db:migrate --name "init"`
- [ ] Verify migration succeeded (tables should be created)
- [ ] Check with `npm run db:studio` to view schema in GUI (look for new social models)

## Verification Phase

- [ ] Type compile: `npm run build` (0 errors)
- [ ] Prisma generate: `npm run db:generate` (✔ success)
- [ ] Start server: `npm run start:dev` (should show PrismaModule initialized)
- [ ] Check server health: `curl http://localhost:3000` (should get response)
- [ ] Open Swagger: Visit `http://localhost:3000/docs` (should load)

## Post-Setup Validation

- [ ] User table exists in database
- [ ] Guild table exists in database
- [ ] GuildMembership table exists in database
- [ ] Bounty table exists in database
- [ ] All foreign key relationships are created
- [ ] Social tables created (Follow, Message, ForumThread, ForumPost, Notification, PrivacySettings)
- [ ] Prisma client types are generated in `node_modules/@prisma/client`

## Ready to Develop

- [ ] Read [QUICK_START.md](QUICK_START.md) for common commands
- [ ] Review [src/user/user.service.ts](src/user/user.service.ts) for example operations
- [ ] Check [prisma/schema.prisma](prisma/schema.prisma) for data model
- [ ] Start building features!

## Troubleshooting Checklist

If something goes wrong:

### PostgreSQL Won't Start
- [ ] Verify Docker is running: `docker ps`
- [ ] Check if port 5432 is in use: `lsof -i :5432`
- [ ] Restart Docker Compose: `docker-compose down && docker-compose up -d`
- [ ] Check logs: `docker-compose logs postgres`

### Prisma Client Won't Generate
- [ ] Delete node_modules: `rm -rf node_modules`
- [ ] Reinstall: `npm install`
- [ ] Try again: `npm run db:generate`

### TypeScript Compilation Fails
- [ ] Clear dist folder: `rm -rf dist`
- [ ] Rebuild: `npm run build`
- [ ] Check for syntax errors in schema.prisma

### Server Won't Start
- [ ] Kill any process on port 3000: `lsof -i :3000 | grep node | awk '{print $2}' | xargs kill`
- [ ] Verify `.env` file exists and has DATABASE_URL
- [ ] Try starting again: `npm run start:dev`

### Database Connection Fails
- [ ] Verify `.env` DATABASE_URL is correct
- [ ] Confirm PostgreSQL is running: `docker-compose ps`
- [ ] Check PostgreSQL logs: `docker-compose logs -f postgres`
- [ ] Try reset: `npx prisma db push --skip-generate`

## Quick Reference Commands

```bash
# Daily development commands
npm run start:dev          # Start server in watch mode
npm run db:studio          # Open database GUI
npm run build              # Verify TypeScript builds

# When you change schema.prisma
npm run db:migrate         # Create and run migration
npm run db:generate        # Regenerate Prisma client

# Database utilities
docker-compose up -d       # Start database
docker-compose down        # Stop database
docker-compose logs -f     # View database logs

# Testing
npm run test              # Run unit tests
npm run test:e2e          # Run E2E tests
npm run lint              # Run ESLint
```

## Important Notes

⚠️ **Do NOT** commit `.env` file (it contains credentials)
⚠️ **Do NOT** run `npx prisma migrate reset` in production
⚠️ **Always** run migrations before deploying
⚠️ **Always** keep `prisma/schema.prisma` in sync with your codebase

---

**Status**: All systems ready for development! ✅

**Maintained by**: DevOps Team  
**Last Updated**: 2026-01-26
