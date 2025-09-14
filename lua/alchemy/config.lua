-- Configuration module for alchemy plugin
-- Handles setup and user configuration options

local M = {}

-- Get the path to the plugin directory
local function get_plugin_dir()
  local source = debug.getinfo(1, "S").source:sub(2)
  return vim.fn.fnamemodify(source, ":p:h:h:h")
end

-- Default configuration
M.defaults = {
  -- CLI configuration
  cli = {
    bin = get_plugin_dir() .. "/bin/alchemy",  -- Use plugin's local binary
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