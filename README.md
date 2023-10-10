This GitHub repository contains a Rust program developed using the Anchor framework, which is designed for creating Solana smart contracts. The code in this repository implements a simple voting system on the Solana blockchain. Here's an overview of the key components and functionality:

    Program Entry Points: The code defines two program entry points, initialize and upvote, and downvote, which are callable functions to interact with the voting system on the Solana blockchain.

    Initialize Function: The initialize function initializes a new voting account. It sets the initial vote count to 0 and generates a unique bump value for the account. This function is used to create a new voting account.

    Upvote Function: The upvote function allows users to increment the vote count of a specific voting account. Each upvote increases the vote count by 1.

    Downvote Function: The downvote function allows users to decrement the vote count of a specific voting account. Each downvote decreases the vote count by 1.

    Account Structures: Two custom account structures, Initialize and VoteInteraction, are defined to interact with the Solana blockchain. These structures hold necessary account information and are used as arguments in the program's entry points.

    Vote Account: The Vote struct represents a voting account. It contains two fields, votes to store the vote count and bump to store a unique identifier for the account.

    Anchor Framework: This code uses the Anchor framework's prelude for working with Solana smart contracts, simplifying the development process.
