use std::{collections::HashSet, fmt::{Display, format, Debug}};
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::node;
use tokio::sync::mpsc;