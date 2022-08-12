local reactor_ok, reactor = pcall(require, 'reactor')
if not reactor_ok then
  return
end

M = {}

local _palette = reactor.get_palette()
M.ReactorPalette = {
  black         = _palette[1],
  red           = _palette[2],
  green         = _palette[3],
  yellow        = _palette[4],
  blue          = _palette[5],
  magenta       = _palette[6],
  cyan          = _palette[7],
  white         = _palette[8],
  light_gray    = _palette[9],
  light_red     = _palette[10],
  light_green   = _palette[11],
  light_yellow  = _palette[12],
  light_blue    = _palette[13],
  light_magenta = _palette[14],
  light_cyan    = _palette[15],
  light_white   = _palette[16],
}

M.LualineTheme = {
  -- Normal Mode
  normal = {
    a = {
      bg = M.ReactorPalette.red,
      fg = M.ReactorPalette.light_white,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.green,
      gui = 'bold'
    },
    c = {
      bg = M.ReactorPalette.black,
      fg = M.ReactorPalette.light_white
    }
  },

  -- Insert Mode
  insert = {
    a = {
      bg = M.ReactorPalette.blue,
      fg = M.ReactorPalette.black,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.white
    },
    c = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.light_white
    }
  },

  -- Visual Mode
  visual = {
    a = {
      bg = M.ReactorPalette.yellow,
      fg = M.ReactorPalette.black,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.white
    },
    c = {
      bg = M.ReactorPalette.blue,
      fg = M.ReactorPalette.black
    }
  },

  -- Replace Mode
  replace = {
    a = {
      bg = M.ReactorPalette.red,
      fg = M.ReactorPalette.black,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.white
      },
    c = {
      bg = M.ReactorPalette.black,
      fg = M.ReactorPalette.white
      }
  },

  -- Command Mode
  command = {
    a = {
      bg = M.ReactorPalette.green,
      fg = M.ReactorPalette.black,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.light_gray,
      fg = M.ReactorPalette.white
      },
    c = {
      bg = M.ReactorPalette.blue,
      fg = M.ReactorPalette.black
      }
  },

  -- Inactive Mode
  inactive = {
    a = {
      bg = M.ReactorPalette.black,
      fg = M.ReactorPalette.gray,
      gui = 'bold'
    },
    b = {
      bg = M.ReactorPalette.black,
      fg = M.ReactorPalette.gray
    },
    c = {
      bg = M.ReactorPalette.black,
      fg = M.ReactorPalette.gray
    }
  }
}

return M
