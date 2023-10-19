# Duviri Mood Clock

## Overview

This version uses the chrono crate and applies my growing knowledge of how to extract values from Ok().

If you're interested in the developer notes, feel free to peruse the older commits!

I'm still learning to use Rust, so if you spot anything weird, please let me know via issues, or PR if it's especially egregious.

## Usage

The current iteration is a commandline program. The arguments available are as follows:

| arg | Description |
| --- | ----------- |
| `query DATETIME` | DATE needs to be in the following format: `%Y-%m-%dT%T%Z:00` or `%Y-%m-%dT%T%:z` |
|                | - bash: date "+%Y-%m-%dT%T%:z" |
|                | - Powershell: Get-Date -UFormat "%Y-%m-%dT%T%Z:00" |
| | DATE can also be `now` |
| | - example: `query now` |


## Overall Goal

This will eventually be a webpage that can:
- display the information
- allow for users to suggest new seed time and mood, in case updates or game server issues changes the mood spiral schedule

## License

GNU General Public License v3.0 (SPDX: GPL-3.0-or-later)

At first I was going to go with the Unlicense when I was going to use primitives. But now I'm choosing to go with a more Rust(-ic? -y?) approach. There is more logic thrown into this than I initially planned as well.

This license *should* allow you to host your own version of the mood clock (in case something happens to my own version). If not, please feel free to correct me and make a PR for a more suitable license!