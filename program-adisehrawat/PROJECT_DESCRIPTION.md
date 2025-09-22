# Project Description

**Deployed Frontend URL:** [https://solearnme.vercel.app]

**Solana Program ID:** 3J4pJELCCwVFjD58iBUUa46pmrZNXwkWGwQkYm8pAc4j

## Project Overview

### Description
solEARN is a decentralized bounty platform built on Solana that connects talented creators with clients seeking specific work. The platform enables clients to post bounties with SOL rewards, while creators can submit their work and get paid instantly through smart contracts when their submissions are selected. The dApp provides a seamless experience for both clients and creators, with automated escrow management and instant payment processing.

### Key Features
- **Dual User Roles**: Support for both clients (bounty posters) and creators (bounty hunters)
- **Smart Contract Escrow**: Automated SOL escrow system for secure bounty rewards
- **Bounty Management**: Create, update, and delete bounties with specific requirements and deadlines
- **Submission System**: Creators can submit work for bounties with descriptions and work URLs
- **Instant Payments**: Automated reward distribution when submissions are selected
- **Skill-Based Matching**: Skill requirements for bounties and skill-based filtering
- **Real-time Analytics**: Track earnings, submissions, and completion statistics
- **Wallet Integration**: Seamless Solana wallet connection and management

## Program Architecture
The Solana program is built using Anchor framework and implements a comprehensive bounty system with user management, client management, bounty creation, submission handling, and automated payment processing.

### PDA Usage
The program extensively uses Program Derived Addresses (PDAs) for secure account management and deterministic address generation.

**PDAs Used:**
- **User PDA**: `[b"user", authority.key()]` - Stores user profiles with skills, bio, and statistics
- **Client PDA**: `[b"client", authority.key()]` - Stores client company information and bounty statistics
- **Bounty PDA**: `[b"bounty", title.as_bytes(), authority.key()]` - Stores bounty details and requirements
- **Submission PDA**: `[b"submission", user_authority.key(), bounty.key()]` - Stores work submissions for bounties
- **Escrow PDA**: `[b"escrow", bounty.key()]` - Holds SOL rewards in escrow until bounty completion

### Program Instructions
The program implements a complete workflow for bounty management and payment processing.

**Instructions Implemented:**
- **User Management**: `create_user`, `update_user`, `delete_user` - Handle user profile creation and updates
- **Client Management**: `create_client`, `update_client`, `delete_client` - Handle client profile creation and updates
- **Bounty Management**: `create_bounty`, `update_bounty`, `delete_bounty` - Handle bounty lifecycle management
- **Submission Handling**: `create_submission` - Allow creators to submit work for bounties
- **Payment Processing**: `select_submission` - Select winning submissions and distribute rewards automatically

### Account Structure
The program uses well-structured account types with proper validation and space management.

```rust
// User account structure
#[account]
pub struct User {
    pub authority: Pubkey,           // User's wallet public key
    pub name: String,                // User's display name
    pub email: String,               // User's email address
    pub avatar: String,              // Auto-generated avatar initials
    pub bio: String,                 // User's biography
    pub skills: Vec<String>,         // User's skills array
    pub joined_at: u64,              // Timestamp when user joined
    pub earned: u64,                 // Total SOL earned from bounties
    pub bounties_submitted: u64,     // Number of bounties submitted to
    pub bounties_completed: u64,     // Number of bounties completed
    pub bump: u8,                    // PDA bump seed
}

// Bounty account structure
#[account]
pub struct Bounty {
    pub creator_wallet_key: Pubkey,  // Client's wallet address
    pub client_key: Pubkey,          // Client account reference
    pub title: String,               // Bounty title
    pub description: String,         // Detailed bounty description
    pub reward: u64,                 // Reward amount in SOL
    pub live: bool,                  // Whether bounty is active
    pub created_at: u64,             // Creation timestamp
    pub deadline: u64,               // Submission deadline
    pub required_skills: Vec<String>, // Skills needed for the bounty
    pub no_of_submissions: u64,      // Number of submissions received
    pub selected_submission: Pubkey, // Winning submission account
    pub selected_user_wallet_key: Pubkey, // Winner's wallet address
    pub escrow_account: Pubkey,      // Escrow account holding rewards
    pub bounty_rewarded: bool,       // Whether reward has been distributed
    pub bump: u8,                    // PDA bump seed
}

// Client account structure
#[account]
pub struct Client {
    pub authority: Pubkey,           // Client's wallet public key
    pub company_name: String,        // Company name
    pub company_email: String,       // Company email
    pub company_avatar: String,      // Company avatar
    pub company_link: String,        // Company website
    pub company_bio: String,         // Company description
    pub joined_at: u64,              // Timestamp when client joined
    pub rewarded: u64,               // Total SOL rewarded
    pub bounties_posted: u64,        // Number of bounties posted
    pub bump: u8,                    // PDA bump seed
}

// Submission account structure
#[account]
pub struct Submission {
    pub user_wallet_key: Pubkey,     // Creator's wallet address
    pub user_key: Pubkey,            // User account reference
    pub bounty_key: Pubkey,          // Bounty account reference
    pub description: String,         // Submission description
    pub work_url: String,            // Link to submitted work
    pub bump: u8,                    // PDA bump seed
}
```

## Testing

### Test Coverage
The project includes comprehensive testing covering all major functionality with both happy path and error scenarios.

**Happy Path Tests:**
- **User Management**: Create, update, and delete user profiles with various data combinations
- **Client Management**: Create, update, and delete client profiles with company information
- **Bounty Management**: Create bounties with SOL rewards and skill requirements
- **Submission Workflow**: Submit work for bounties and select winning submissions
- **Payment Processing**: Automated reward distribution through escrow accounts
- **Data Validation**: Proper handling of input validation and constraints

**Unhappy Path Tests:**
- **Authorization Errors**: Unauthorized access attempts to update/delete other users' profiles
- **Input Validation**: Empty names, invalid email formats, and oversized skill arrays
- **Bounty Constraints**: Attempting to delete bounties with active submissions
- **Deadline Enforcement**: Submissions after bounty deadlines
- **Insufficient Funds**: Creating bounties without sufficient SOL balance
- **Account Constraints**: PDA seed validation and account ownership verification

### Running Tests
```bash
# Navigate to the solearn directory
cd solearn

# Run all tests
anchor test

# Run tests with extended timeout (recommended)
yarn test
```

### Additional Notes for Evaluators

**Frontend Architecture:**
- Built with React + TypeScript using Vite
- Modern UI components with shadcn/ui and Tailwind CSS
- Responsive design with Framer Motion animations
- Real-time data integration with Solana program

**Security Features:**
- PDA-based account management prevents unauthorized access
- Escrow system ensures secure reward distribution
- Input validation and constraint checking
- Proper authority verification for all operations

**Performance Optimizations:**
- Efficient account space management using InitSpace derive
- Batch operations for bounty management
- Optimized PDA derivation for fast lookups

**User Experience:**
- Seamless wallet connection and role selection
- Intuitive bounty creation and submission workflow
- Real-time updates and notifications
- Comprehensive analytics and statistics tracking

**Deployment Ready:**
- Program ID configured for localnet testing
- Frontend configured for production builds
- Comprehensive error handling and user feedback
- Mobile-responsive design for all devices
