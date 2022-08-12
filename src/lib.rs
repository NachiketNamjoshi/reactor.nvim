use std::collections::HashMap;
pub mod colors;

use nvim_oxi::{self as oxi, api, opts::*};

#[oxi::module]
fn reactor() -> oxi::Result<()> {
    api::set_option("termguicolors", true)?;
    let reactor: HashMap<i32, String> = colors::get_colors();
    let palette: HashMap<i32, String> = colors::get_palette();
    macro_rules! sethl {
        ($hlname: expr, $fgcol: expr, $bgcol: expr) => {
            api::set_hl(
                0,
                stringify!($hlname),
                Some(
                    &SetHighlightOpts::builder()
                        .fg(reactor.get($fgcol).unwrap())
                        .bg(reactor.get($bgcol).unwrap())
                        .build(),
                ),
            )?;
        };
        ($hlname: expr, $fgcol: expr, $bgcol: expr, $key: ident) => {
            api::set_hl(
                0,
                stringify!($hlname),
                Some(
                    &SetHighlightOpts::builder()
                        .fg(reactor.get($fgcol).unwrap())
                        .bg(reactor.get($bgcol).unwrap())
                        .$key(true)
                        .build(),
                ),
            )?;
        };
    }

    api::set_var("terminal_color_background", palette.get(&0).unwrap().to_string())?;
    api::set_var("terminal_color_foreground", palette.get(&15).unwrap().to_string())?;

    api::set_var("terminal_color_0", palette.get(&0).unwrap().to_string())?;
    api::set_var("terminal_color_1", palette.get(&1).unwrap().to_string())?;
    api::set_var("terminal_color_2", palette.get(&2).unwrap().to_string())?;
    api::set_var("terminal_color_3", palette.get(&3).unwrap().to_string())?;
    api::set_var("terminal_color_4", palette.get(&4).unwrap().to_string())?;
    api::set_var("terminal_color_5", palette.get(&5).unwrap().to_string())?;
    api::set_var("terminal_color_6", palette.get(&6).unwrap().to_string())?;
    api::set_var("terminal_color_7", palette.get(&7).unwrap().to_string())?;
    api::set_var("terminal_color_8", palette.get(&8).unwrap().to_string())?;
    api::set_var("terminal_color_9", palette.get(&9).unwrap().to_string())?;
    api::set_var("terminal_color_10", palette.get(&10).unwrap().to_string())?;
    api::set_var("terminal_color_11", palette.get(&11).unwrap().to_string())?;
    api::set_var("terminal_color_12", palette.get(&12).unwrap().to_string())?;
    api::set_var("terminal_color_13", palette.get(&13).unwrap().to_string())?;
    api::set_var("terminal_color_14", palette.get(&14).unwrap().to_string())?;
    api::set_var("terminal_color_15", palette.get(&15).unwrap().to_string())?;

    // Telescope Plugin
    sethl!(TelescopeSelection, &70, &136);
    sethl!(TelescopeMatching, &191, &999, bold);
    sethl!(TelescopeBorder, &102, &0);

    // Basics
    sethl!(Normal, &255, &0);
    sethl!(SignColumn, &999, &1);
    sethl!(MsgArea, &255, &1);
    sethl!(ModeMsg, &255, &1);
    sethl!(MsgSeparator, &255, &1);
    sethl!(SpellBad, &12, &1, underline);
    sethl!(SpellCap, &51, &999, underline);
    sethl!(SpellLocal, &59, &999, underline);
    sethl!(SpellRare, &85, &999, underline);
    sethl!(NormalNC, &255, &0);
    sethl!(Pmenu, &248, &234);
    sethl!(PmenuSel, &0, &73, bold);
    sethl!(WildMenu, &0, &73);
    sethl!(CursorLineNr, &251, &1, bold);
    sethl!(Comment, &120, &999);
    sethl!(Folded, &68, &234);
    sethl!(FoldColumn, &68, &234);
    sethl!(LineNr, &249, &999);
    sethl!(FloatBoder, &249, &234);
    sethl!(Whitespace, &999, &999);
    sethl!(VertSplit, &249, &0);
    sethl!(CursorLine, &999, &1);
    sethl!(CursorColumn, &999, &1);
    sethl!(ColorColumn, &999, &1);
    sethl!(NormalFloat, &999, &1);
    sethl!(Visual, &999, &136);
    sethl!(VisualNOS, &999, &136);
    sethl!(WarningMsg, &1, &0, bold);
    sethl!(DiffAdd, &234, &10);
    sethl!(DiffChange, &234, &11, underline);
    sethl!(DiffDelete, &234, &125);
    sethl!(QuickFixLine, &255, &73);
    sethl!(PmenuSbar, &999, &234);
    sethl!(PmenuThumb, &999, &242);
    sethl!(MatchWord, &999, &999, underline);
    sethl!(MatchParen, &70, &0, underline);
    sethl!(MatchWordCur, &249, &1, underline);
    sethl!(MatchParenCur, &70, &0, underline);
    sethl!(Cursor, &1, &253);
    sethl!(lCursor, &1, &253);
    sethl!(CursorIM, &1, &253);
    sethl!(TermCursor, &1, &253);
    sethl!(TermCursorNC, &1, &253);
    sethl!(Conceal, &68, &0);
    sethl!(Directory, &4, &999);
    sethl!(SpecialKey, &4, &999, bold);
    sethl!(Title, &202, &999, bold);
    sethl!(ErrorMsg, &1, &0, bold);
    sethl!(Search, &250, &60, bold);
    sethl!(IncSearch, &208, &236, bold);
    sethl!(Substitute, &208, &236, underline);
    sethl!(MoreMsg, &173, &999);
    sethl!(Question, &173, &999);
    sethl!(EndOfBuffer, &999, &0);
    sethl!(NonText, &999, &0);
    sethl!(Variable, &6, &999);
    sethl!(String, &60, &999);
    sethl!(Character, &60, &999, bold);
    sethl!(Constant, &73, &999);
    sethl!(Number, &105, &999);
    sethl!(Boolean, &164, &999);
    sethl!(Float, &105, &999);
    sethl!(Identifier, &23, &999);
    sethl!(Function, &186, &999);
    sethl!(Operator, &255, &999);
    sethl!(Type, &108, &999);
    sethl!(StorageClass, &21, &999);
    sethl!(Structure, &36, &999);
    sethl!(Typedef, &108, &999);
    sethl!(Keyword, &38, &999);
    sethl!(Statement, &85, &999);
    sethl!(Conditional, &99, &999);
    sethl!(Repeat, &55, &999);
    sethl!(Label, &246, &999, bold);
    sethl!(Exception, &152, &999);
    sethl!(Include, &17, &999);
    sethl!(PreProc, &14, &999, bold);
    sethl!(Define, &86, &999, bold);
    sethl!(Macro, &196, &999, bold);
    sethl!(PreCondit, &14, &999);
    sethl!(Special, &166, &999, bold);
    sethl!(SpecialChar, &255, &999);
    sethl!(Tag, &109, &1);
    sethl!(Debug, &50, &5);
    sethl!(Delimiter, &249, &999);
    sethl!(SpecialComment, &120, &999, bold);
    sethl!(Ignore, &7, &999, bold);
    sethl!(Todo, &187, &4, bold);
    sethl!(Error, &15, &4, bold);
    sethl!(TabLine, &250, &2);
    sethl!(TabLineSel, &250, &2, bold);
    sethl!(TabLineFill, &249, &2);
    Ok(())
}