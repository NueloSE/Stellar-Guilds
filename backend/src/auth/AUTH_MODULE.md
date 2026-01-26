# Authentication Module Documentation

## Overview

The Authentication Module provides a secure, production-ready authentication system for the Stellar Guilds backend. It supports both traditional email/password authentication and Web3 wallet signature verification for blockchain integration.

### Key Features

- **JWT Token-Based Authentication**: Industry-standard JWT tokens for stateless authentication
- **Web3 Wallet Integration**: Sign-in with Ethereum wallet signatures using ethers.js
- **Refresh Token Rotation**: Secure token refresh mechanism with extended expiration
- **Password Hashing**: bcrypt with salt rounds for secure password storage
- **Protected Routes**: JWT guard for protecting sensitive endpoints
- **User Account Linking**: Link wallet addresses to existing accounts
- **Session Management**: Logout functionality with token invalidation

---

## Architecture

### Components

#### 1. **AuthService** (`auth.service.ts`)
Core service handling all authentication logic:
- User registration with email/password
- Login with credentials
- Web3 wallet authentication
- Token generation and validation
- Token refresh mechanism
- User profile retrieval

#### 2. **AuthController** (`auth.controller.ts`)
REST API endpoints:
- `POST /auth/register` - Register new user
- `POST /auth/login` - Login with email/password
- `POST /auth/wallet-auth` - Authenticate with wallet signature
- `POST /auth/refresh` - Refresh access token
- `POST /auth/logout` - Logout user (protected)
- `GET /auth/me` - Get current user profile (protected)

#### 3. **JWT Strategy** (`strategies/jwt.strategy.ts`)
Passport.js JWT strategy implementation for validating JWT tokens in requests.

#### 4. **JWT Auth Guard** (`guards/jwt-auth.guard.ts`)
NestJS guard for protecting routes that require authentication.

#### 5. **Auth DTOs** (`dto/auth.dto.ts`)
Data validation classes:
- `RegisterDto` - Registration request validation
- `LoginDto` - Login request validation
- `WalletAuthDto` - Wallet authentication validation
- `RefreshTokenDto` - Token refresh validation
- `AuthResponseDto` - Response format

---

## API Endpoints

### 1. Register User
```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "username",
  "password": "SecurePassword123",
  "firstName": "John",
  "lastName": "Doe",
  "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f1e2f0" // Optional
}

Response: 201 Created
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "clx1234...",
    "email": "user@example.com",
    "username": "username",
    "walletAddress": "0x742d35..."
  }
}
```

### 2. Login User
```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123"
}

Response: 200 OK
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "clx1234...",
    "email": "user@example.com",
    "username": "username",
    "walletAddress": null
  }
}
```

### 3. Wallet Authentication (Web3)
```http
POST /auth/wallet-auth
Content-Type: application/json

{
  "walletAddress": "0x742d35Cc6634C0532925a3b844Bc9e7595f1e2f0",
  "message": "Welcome to Stellar Guilds!\nSign to verify ownership",
  "signature": "0x1234567890abcdef..."
}

Response: 200 OK
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "clx1234...",
    "email": "user_1234@wallet.local",
    "username": "user_1234",
    "walletAddress": "0x742d35..."
  }
}
```

### 4. Refresh Access Token
```http
POST /auth/refresh
Content-Type: application/json

{
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}

Response: 200 OK
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "clx1234...",
    "email": "user@example.com",
    "username": "username",
    "walletAddress": null
  }
}
```

### 5. Get Current User Profile (Protected)
```http
GET /auth/me
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

Response: 200 OK
{
  "id": "clx1234...",
  "email": "user@example.com",
  "username": "username",
  "firstName": "John",
  "lastName": "Doe",
  "walletAddress": null,
  "role": "USER",
  "isActive": true,
  "createdAt": "2024-01-26T10:00:00Z"
}
```

### 6. Logout (Protected)
```http
POST /auth/logout
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

Response: 200 OK
{
  "message": "Logged out successfully"
}
```

---

## Token Structure

### JWT Payload
```json
{
  "sub": "clx1234...",          // User ID
  "email": "user@example.com",  // User email
  "walletAddress": "0x742d...", // Optional: User's wallet address
  "iat": 1704825600,            // Issued at
  "exp": 1704826500             // Expires in (15 minutes for access token)
}
```

### Token Expiration
- **Access Token**: 15 minutes (900 seconds)
- **Refresh Token**: 7 days (604800 seconds)

---

## Using Protected Routes

### 1. Import the Guard
```typescript
import { JwtAuthGuard } from './auth/guards/jwt-auth.guard';
```

### 2. Apply to Routes
```typescript
@Controller('api/protected')
export class ProtectedController {
  
  @Get('resource')
  @UseGuards(JwtAuthGuard)
  getResource(@Request() req) {
    const userId = req.user.userId;
    const email = req.user.email;
    // Handle request
  }
}
```

### 3. Access User Information
```typescript
@UseGuards(JwtAuthGuard)
@Post('action')
performAction(@Request() req) {
  console.log(req.user); // { userId, email, walletAddress }
}
```

---

## Web3 Wallet Authentication Flow

### Client-Side Example

```typescript
import { ethers } from 'ethers';

async function walletLogin(walletAddress: string) {
  // 1. Create message to sign
  const message = "Welcome to Stellar Guilds!\nSign to verify ownership";
  
  // 2. Request signature from user's wallet
  const provider = new ethers.BrowserProvider(window.ethereum);
  const signer = await provider.getSigner();
  const signature = await signer.signMessage(message);
  
  // 3. Send to backend for verification
  const response = await fetch('/auth/wallet-auth', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      walletAddress,
      message,
      signature
    })
  });
  
  const { accessToken, refreshToken } = await response.json();
  
  // 4. Store tokens (localStorage, sessionStorage, or secure cookie)
  localStorage.setItem('accessToken', accessToken);
  localStorage.setItem('refreshToken', refreshToken);
}
```

---

## Configuration

### Environment Variables

Add these to your `.env` file:

```env
# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
REFRESH_TOKEN_SECRET=your-super-secret-refresh-key-min-32-chars

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/stellar_guilds

# Optional: Override default token expiry (in JWT time format: 15m, 7d, etc)
JWT_EXPIRY=15m
REFRESH_TOKEN_EXPIRY=7d
```

### Security Best Practices

1. **Use strong secrets** - Generate 32+ character random strings for JWT secrets
2. **HTTPS only** - Always use HTTPS in production
3. **Secure cookies** - Store tokens in httpOnly, secure cookies when possible
4. **CORS configuration** - Restrict CORS origins in production
5. **Rate limiting** - Implement rate limiting on auth endpoints
6. **Input validation** - All DTOs use class-validator for validation
7. **Password policies** - Enforce strong password requirements
8. **Token rotation** - Use refresh tokens for automatic token rotation

---

## Testing

### Run Auth Tests
```bash
npm test -- auth
```

### Test Coverage
- Unit tests for all auth service methods
- Integration tests for controller endpoints
- Tests for token generation and validation
- Tests for wallet signature verification
- Tests for error handling and edge cases

### Example Test
```typescript
describe('AuthService', () => {
  it('should register a new user successfully', async () => {
    const registerDto = {
      email: 'test@example.com',
      username: 'testuser',
      password: 'Password123',
      firstName: 'Test',
      lastName: 'User'
    };

    const result = await authService.register(registerDto);

    expect(result.accessToken).toBeDefined();
    expect(result.refreshToken).toBeDefined();
    expect(result.user.email).toEqual(registerDto.email);
  });
});
```

---

## Error Handling

### Common Error Responses

#### 400 Bad Request
```json
{
  "statusCode": 400,
  "message": "Invalid wallet address",
  "error": "Bad Request"
}
```

#### 401 Unauthorized
```json
{
  "statusCode": 401,
  "message": "Invalid email or password",
  "error": "Unauthorized"
}
```

#### 409 Conflict
```json
{
  "statusCode": 409,
  "message": "User with this email or username already exists",
  "error": "Conflict"
}
```

---

## Module Integration

The AuthModule is already integrated into the main AppModule:

```typescript
// app.module.ts
import { AuthModule } from './auth/auth.module';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    PrismaModule,
    AuthModule,  // ← Authentication module
    UserModule,
  ],
})
export class AppModule {}
```

---

## Security Considerations

1. **Password Hashing**: Uses bcrypt with 10 salt rounds
2. **Signature Verification**: Uses ethers.js `verifyMessage()` for cryptographic verification
3. **Token Secrets**: Should be 32+ characters and randomly generated
4. **Refresh Token Storage**: Stored in database and validated on each refresh
5. **Input Validation**: All inputs validated against DTOs with class-validator
6. **SQL Injection Prevention**: Uses Prisma ORM with parameterized queries
7. **XSS Protection**: Token handled server-side, consider httpOnly cookies

---

## Troubleshooting

### "Invalid signature" error
- Ensure the message and signature match
- Verify wallet address is properly checksummed
- Check that signature was generated with the same message text

### "Invalid refresh token" error
- Refresh token may have expired (7 days)
- User may have logged out (token invalidated in DB)
- Token may be corrupted or modified

### "User not found" on /auth/me
- Access token may be expired
- User account may have been deleted
- Invalid or tampered token

---

## Future Enhancements

- [ ] Two-factor authentication (2FA)
- [ ] Social login integration (Google, GitHub, etc.)
- [ ] Session management and device tracking
- [ ] Password reset functionality
- [ ] Email verification
- [ ] Account lockout after failed attempts
- [ ] OAuth2 / OpenID Connect support
- [ ] API key generation for programmatic access

---

## References

- [NestJS Authentication](https://docs.nestjs.com/security/authentication)
- [Passport.js JWT Strategy](http://www.passportjs.org/packages/passport-jwt/)
- [ethers.js Message Signing](https://docs.ethers.org/v6/api/utils/#signing)
- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)
