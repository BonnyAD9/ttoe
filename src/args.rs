use std::io::{self, IsTerminal};

use pareg::{ArgError, ArgIterator, ByRef};
use termal::raw;

use crate::{err::Result, vec2::Vec2};

#[derive(Default)]
pub struct Args {
    help: bool,
    size: Option<Vec2>,
    win_len: Option<usize>,
    use_color: Option<bool>,
}

impl Args {
    pub fn help(&self) -> bool {
        self.help
    }

    pub fn size(&self) -> Vec2 {
        self.size.unwrap_or((15, 15).into())
    }

    pub fn win_len(&self) -> usize {
        self.win_len.unwrap_or_else(|| self.size().cmax().min(5))
    }

    pub fn color(&self) -> bool {
        self.use_color.unwrap_or(io::stdout().is_terminal())
    }

    pub fn parse<'a, I, A>(mut args: ArgIterator<'a, I>) -> Result<Self>
    where
        I: Iterator<Item = A>,
        A: ByRef<&'a str>,
    {
        let mut res = Self::default();

        args.next();

        res.parse_self(args)?;

        res.finalize();
        Ok(res)
    }

    pub fn parse_self<'a, I, A>(
        &mut self,
        mut args: ArgIterator<'a, I>,
    ) -> Result<()>
    where
        I: Iterator<Item = A>,
        A: ByRef<&'a str>,
    {
        while let Some(arg) = args.next() {
            match arg {
                "-h" | "-?" | "--help" => self.help = true,
                "-s" | "--size" => {
                    let size: Vec2 =
                        args.next_key_val::<usize, usize>('x')?.into();
                    if size.cmin() == 0 {
                        Err(ArgError::FailedToParse {
                            typ: "size",
                            value: args.cur_arg::<&str>()?.to_owned().into(),
                            msg: Some("The size must not be 0.".into()),
                        })?;
                    }
                    self.size = Some(size);
                }
                "-w" | "--win" | "--win-length" => {
                    let wl = args.next_arg()?;
                    if wl == 0 {
                        Err(ArgError::FailedToParse {
                            typ: "length",
                            value: args.cur_arg::<&str>()?.to_owned().into(),
                            msg: Some("The win length cannot be 0.".into()),
                        })?;
                    }
                    self.win_len = Some(wl);
                }
                "--color" | "--colour" => {
                    self.use_color =
                        args.next_opt_bool("always", "never", "auto")?;
                }
                _ => Err(ArgError::UnknownArgument(arg.to_owned().into()))?,
            }
        }

        Ok(())
    }

    pub fn finalize(&mut self) {
        if self.size.is_some() {
            return;
        }

        let Ok(size) = raw::term_size() else {
            return;
        };

        let size: Vec2 = (size.char_width, size.char_height).into();
        self.size = Some((size - (1, 2)).cdiv((4, 2)).max((1, 1)));
    }
}
