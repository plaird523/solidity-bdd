Feature: Basic Auction
  As the creator of a basic NFT
  In order to gain compensation for my creation
  I want to sell my NFT through the Zora marketplace

  Background:
    Given the following wallet addresses and balances:
      | Name   | Amount |
      | George | 1000   |
      | Susan  | 1000   |

  Scenario: Reserve Auction Starts
    Given I have minted a basic NFT with tokenId 1
    When I create a Zora listing for tokenId 1 with reserve price of 100 Finney and duration of 3 days
    Then there should be an active auction for tokenId 1 with reserve price of 100 Finney and duration of 3 days
    And the owner of tokenId 1 should be the minting address

  Scenario: Reserve Auction Receives Minimum Bid
    Given I have minted a basic NFT with tokenId 1
    And I have created a Zora listing for tokenId 1 with reserve price of 100 Finney and duration of 3 days
    When George places a bid of 200 Finney
    Then there should be an active auction for tokenId 1 with high bid of 200 Finney and high bidder of George
    And the owner of tokenID 1 should be the Zora contract

  Scenario: Reserve Auction Completes with Minimum Bid Met
    Given I have minted a basic NFT with tokenId 1
    And I have created a Zora listing for tokenId 1 with reserve price of 100 Finney and duration of 3 days
    And Susan has placed a bid of 150 Finney
    And 4 days have elapsed since the auction was created
    When I close the auction
    Then Susan should be the owner of tokenId 1
    And the minter's balance should be 1150 Finney
    And Susan's balance should be 850 Finney

  Scenario: Reserve Auction Completes without Minimum Bid
    Given I have minted a basic NFT with tokenId 1
    And I have created a Zora listing for tokenId 1 with reserve price of 100 Finney and duration of 3 days
    And there have been no bids
    And 4 days have elapsed since the auction was created
    When I close the auction
    Then the minter should be the owner of tokenId 1
    And the minter's balance should be 1000 Finney