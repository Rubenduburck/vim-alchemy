local M = {}

function M.check()
    local health = vim.health or require("health")
    local Config = require("alchemy.config")
    
    health.start("vim-alchemy")
    
    -- Check Neovim version
    if vim.fn.has("nvim-0.8") == 1 then
        health.ok("Neovim version >= 0.8")
    else
        health.error("Neovim version < 0.8", "Please update Neovim")
    end
    
    -- Check alchemy binary
    local cli_path = Config.options.cli.bin or "alchemy"
    local handle = io.popen(cli_path .. " --version 2>&1")
    
    if handle then
        local output = handle:read("*a")
        local success = handle:close()
        
        if success and output:match("alchemy") then
            health.ok("alchemy binary found: " .. vim.trim(output))
        else
            health.error("alchemy binary not working properly", {
                "Run 'make install' in the plugin directory",
                "Or download from https://github.com/rubenduburck/alchemy/releases"
            })
        end
    else
        health.error("alchemy binary not found", {
            "Run 'make install' in the plugin directory",
            "Or add alchemy to your PATH"
        })
    end
    
    -- Check optional dependencies
    local has_telescope = pcall(require, "telescope")
    if has_telescope then
        health.ok("telescope.nvim found (optional)")
    else
        health.info("telescope.nvim not found (optional)")
    end
end

return M