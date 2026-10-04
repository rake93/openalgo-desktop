//! OAuth / redirect brokers (arrow, paytm, pocketful, hdfcsky,
//! hdfcsecurities) end to end against local fake brokers on ephemeral
//! ports: sign-in, orders, books, funds, quotes and master downloads.

#[path = "brokers_oauth_batch/hdfcsecurities.rs"]
mod hdfcsecurities;
