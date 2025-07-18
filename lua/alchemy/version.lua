-- Internal version compatibility checking for vim-alchemy
-- This is completely transparent to users
local M = {}

-- Read the required alchemy version from .alchemy-version file
local function get_required_version()
    local plugin_dir = vim.fn.fnamemodify(debug.getinfo(1, "S").source:sub(2), ":p:h:h:h")
    local version_file = plugin_dir .. "/.alchemy-version"
    
    local f = io.open(version_file, "r")
    if not f then
        return nil
    end
    
    local version = f:read("*l")
    f:close()
    return version
end

-- Check if the installed binary matches what we expect
function M.verify_installation()
    local Config = require("alchemy.config")
    local cli_path = Config.options.cli.bin or "alchemy"
    local required_version = get_required_version()
    
    if not required_version or required_version == "latest" then
        -- No specific version required, any working binary is fine
        return true
    end
    
    -- Check if binary exists and works
    local handle = io.popen(cli_path .. " --version 2>&1")
    if not handle then
        return false
    end
    
    local output = handle:read("*a")
    local success = handle:close()
    
    if not success then
        -- Binary doesn't exist or doesn't work
        -- The plugin will handle this by prompting to install
        return false
    end
    
    -- For now, we just check that the binary works
    -- In the future, we could check specific version compatibility
    return true
end

return M