# ─── Stage 1: Dependency installation ───────────────────────────────────────
FROM node:20-alpine AS deps
RUN apk add --no-cache libc6-compat

WORKDIR /app

# Copy package manifests
COPY package.json pnpm-lock.yaml ./

# Install pnpm and frozen-lockfile deps (production + dev needed for build)
RUN npm install -g pnpm@10.28.1 --ignore-scripts && \
    pnpm install --frozen-lockfile

# ─── Stage 2: Build ──────────────────────────────────────────────────────────
FROM node:20-alpine AS builder

WORKDIR /app

# Carry over node_modules from deps stage
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Disable Next.js telemetry inside the container
ENV NEXT_TELEMETRY_DISABLED=1

# Build args injected at image-build time (non-secret public vars)
ARG NEXT_PUBLIC_STELLAR_NETWORK=testnet
ARG NEXT_PUBLIC_HORIZON_URL=https://horizon-testnet.stellar.org
ARG NEXT_PUBLIC_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
ARG NEXT_PUBLIC_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
ARG NEXT_PUBLIC_APP_URL=https://harvesta.app

ENV NEXT_PUBLIC_STELLAR_NETWORK=$NEXT_PUBLIC_STELLAR_NETWORK \
    NEXT_PUBLIC_HORIZON_URL=$NEXT_PUBLIC_HORIZON_URL \
    NEXT_PUBLIC_SOROBAN_RPC_URL=$NEXT_PUBLIC_SOROBAN_RPC_URL \
    NEXT_PUBLIC_NETWORK_PASSPHRASE=$NEXT_PUBLIC_NETWORK_PASSPHRASE \
    NEXT_PUBLIC_APP_URL=$NEXT_PUBLIC_APP_URL

# Build with webpack (matches package.json "build" script)
RUN npm install -g pnpm@10.28.1 --ignore-scripts && \
    pnpm run build

# ─── Stage 3: Production runner ──────────────────────────────────────────────
FROM node:20-alpine AS runner

WORKDIR /app

# Security: run as non-root user
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    PORT=3000 \
    HOSTNAME=0.0.0.0

# Copy only the build artefacts needed at runtime
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

# Healthcheck uses the /api/health endpoint (fast, no DB)
HEALTHCHECK --interval=15s --timeout=5s --start-period=30s --retries=3 \
  CMD wget -qO- http://localhost:3000/api/health || exit 1

CMD ["node", "server.js"]
