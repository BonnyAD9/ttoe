use std::io::{self, IsTerminal};

use pareg::{ArgError, ArgIterator, ByRef};

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
        self.win_len.unwrap_or_else(|| self.size().max().min(5))
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
                    if size.min() == 0 {
                        Err(ArgError::FailedToParse {
                            typ: "Vec2",
                            value: args.cur_arg::<&str>()?.to_owned().into(),
                            msg: Some("The size must not be 0".into()),
                        })?;
                    }
                    self.size = Some(size);
                }
                "-w" | "--win" | "--win-length" => {
                    self.win_len = Some(args.next_arg()?)
                }
                "--color" | "--colour" => {
                    self.use_color = Some(args.next_bool("always", "never")?)
                }
                _ => Err(ArgError::UnknownArgument(arg.to_owned().into()))?,
            }
        }

        Ok(())
    }
}
