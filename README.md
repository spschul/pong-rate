# pong-rate
A ranking system for ping-pong

This is just a for-fun Rust project for ranking players in ping-pong. It currently uses the Elo rating system.

Usage: pong-rate [players file] [matches file]

[players file]: the path to a file containing the names of all the players, one per line

[matches file]: the path to a file containing the results of the matches, where each line has the format:

`winner_name, loser_name`

Currently the timestamp is not used.

All players default to a ranking of 1600, and the current K-factor is 32.
