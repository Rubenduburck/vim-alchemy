-- Configuration module for alchemy plugin
-- Handles setup and user configuration options

local M = {}

-- Default configuration
M.defaults = {
  -- CLI configuration
  cli = {
    bin = "alchemy",  -- Path to alchemy binary
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