/*
 CAP { .. } => 0,
  AUTHENTICATE { .. } => 1,
  PASS { .. } => 2,
  NICK { .. } => 3,
  USER { .. } => 4,
  PING { .. } => 5,
  PONG { .. } => 6,
  OPER { .. } => 7,
  QUIT { .. } => 8,
  JOIN { .. } => 9,
  PART { .. } => 10,
  TOPIC { .. } => 11,
  NAMES { .. } => 12,
  LIST { .. } => 13,
  INVITE { .. } => 14,
  KICK { .. } => 15,
  MOTD { .. } => 16,
  VERSION { .. } => 17,
  ADMIN { .. } => 18,
  CONNECT { .. } => 19,
  LUSERS { .. } => 20,
  TIME { .. } => 21,
  STATS { .. } => 22,
  LINKS { .. } => 23,
  HELP { .. } => 24,
  INFO { .. } => 25,
  MODE { .. } => 26,
  PRIVMSG { .. } => 27,
  NOTICE { .. } => 28,
  WHO { .. } => 29,
  WHOIS { .. } => 30,
  WHOWAS { .. } => 31,
  KILL { .. } => 32,
  REHASH { .. } => 33,
  RESTART { .. } => 34,
  SQUIT { .. } => 35,
  AWAY { .. } => 36,
  USERHOST { .. } => 37,
  WALLOPS { .. } => 38,
  ISON { .. } => 39,
  DIE { .. } => 40,
*/
// 📦 Export module
mod nick;
mod join;
mod privmsg;
mod ping;
mod mode;

pub use nick::nick_command;
pub use join::join_command;
pub use privmsg::privmsg_command;
pub use ping::ping_command;
pub use mode::mode_command;
