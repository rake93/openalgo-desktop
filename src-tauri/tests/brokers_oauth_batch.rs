//! OAuth / redirect brokers (arrow, paytm, pocketful, hdfcsky,
//! hdfcsecurities) end to end against local fake brokers on ephemeral
//! ports: sign-in, orders, books, funds, quotes, history, master contracts.

#[path = "brokers_oauth_batch/hdfcsky.rs"]
mod hdfcsky;
