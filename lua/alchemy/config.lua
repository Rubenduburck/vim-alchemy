-- Configuration module for alchemy plugin
-- Handles setup and user configuration options

local M = {}

-- Default configuration
M.defaults = {
  -- CLI configuration
  cli = {
    bin = "alchemy",  -- Path to alchemy binary
  },
  
  -- UI configuration
  ui = {
    -- Window transparency (0-100)
    winblend = 10,
    
    -- Border style: 'single', 'double', 'rounded', 'solid'
    border = "single",
    
    -- Enable icons (requires a Nerd Font)
    icons = true,
    
    -- Animation speed (ms)
    animation_speed = 150,
  },
  
  -- Default keymaps enabled
  default_keymaps = true,
}

-- Current configuration (starts with defaults)
M.options = vim.deepcopy(M.defaults)

-- Setup function to merge user config with defaults
function M.setup(opts)
  opts = opts or {}
  M.options = vim.tbl_deep_extend("force", M.defaults, opts)
end

return M