-- New modern alchemy plugin initialization
-- Provides intuitive commands with excellent UX

local M = {}

local Commands = require("alchemy.commands")
local Telescope = require("alchemy.telescope")

-- Setup function for configuration
function M.setup(opts)
	opts = opts or {}
	require("alchemy.config").setup(opts)
	M.create_commands()
	
	-- Create keymaps unless explicitly disabled (default: true)
	if opts.default_keymaps ~= false then
		M.create_keymaps()
	end
end

-- Create all user commands
function M.create_commands()
	-- Main conversion command - works with visual selection or word under cursor
	vim.api.nvim_create_user_command("AlchConvert", function(cmd_opts)
		Commands.convert(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Convert text between encodings (auto-detects input, prompts for output if not specified)",
		complete = function(_, _, _)
			return { "hex", "int", "base64", "base58", "bin", "utf8", "ascii", "bytes" }
		end,
	})

	-- Classification explorer - browse classifications and conversions
	vim.api.nvim_create_user_command("AlchExplore", function()
		Commands.explore()
	end, {
		range = true,
		desc = "Explore all classifications and conversion options",
	})

	-- Classification only - show what formats the text could be
	vim.api.nvim_create_user_command("AlchClassify", function()
		Commands.classify()
	end, {
		range = true,
		desc = "Show all possible classifications for selected text",
	})

	-- Quick conversion commands for common formats
	vim.api.nvim_create_user_command("AlchToHex", function()
		Commands.to_hex()
	end, {
		range = true,
		desc = "Convert selected text to hexadecimal",
	})

	vim.api.nvim_create_user_command("AlchToInt", function()
		Commands.to_int()
	end, {
		range = true,
		desc = "Convert selected text to integer",
	})

	vim.api.nvim_create_user_command("AlchToBase64", function()
		Commands.to_base64()
	end, {
		range = true,
		desc = "Convert selected text to base64",
	})

	vim.api.nvim_create_user_command("AlchToBase58", function()
		Commands.to_base58()
	end, {
		range = true,
		desc = "Convert selected text to base58",
	})

	vim.api.nvim_create_user_command("AlchToBin", function()
		Commands.to_bin()
	end, {
		range = true,
		desc = "Convert selected text to binary",
	})

	vim.api.nvim_create_user_command("AlchToUtf8", function()
		Commands.to_utf8()
	end, {
		range = true,
		desc = "Convert selected text to UTF-8",
	})

	vim.api.nvim_create_user_command("AlchToAscii", function()
		Commands.to_ascii()
	end, {
		range = true,
		desc = "Convert selected text to ASCII",
	})

	vim.api.nvim_create_user_command("AlchToBytes", function()
		Commands.to_bytes()
	end, {
		range = true,
		desc = "Convert selected text to byte array",
	})

	-- Array manipulation commands
	vim.api.nvim_create_user_command("AlchFlatten", function()
		Commands.flatten()
	end, {
		range = true,
		desc = "Flatten nested arrays",
	})

	vim.api.nvim_create_user_command("AlchChunk", function(cmd_opts)
		Commands.chunk(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Split array into chunks of specified size",
	})

	vim.api.nvim_create_user_command("AlchRotate", function(cmd_opts)
		Commands.rotate(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Rotate array elements by specified amount",
	})

	-- Padding commands
	vim.api.nvim_create_user_command("AlchPadLeft", function(cmd_opts)
		Commands.pad_left(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Pad array/data on the left to specified size",
	})

	vim.api.nvim_create_user_command("AlchPadRight", function(cmd_opts)
		Commands.pad_right(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Pad array/data on the right to specified size",
	})

	-- Hashing commands
	vim.api.nvim_create_user_command("AlchHash", function(cmd_opts)
		Commands.hash(cmd_opts.fargs)
	end, {
		nargs = "?",
		range = true,
		desc = "Hash selected text with specified algorithm",
		complete = function(_, _, _)
			return { "sha256", "sha512", "md5", "blake2", "keccak256" }
		end,
	})

	-- Generation commands
	vim.api.nvim_create_user_command("AlchGenerate", function(cmd_opts)
		Commands.generate(cmd_opts.fargs)
	end, {
		nargs = "*",
		range = true,
		desc = "Generate random data in specified format and length",
		complete = function(_, _, _)
			return { "hex", "base64", "base58", "bin", "int", "utf8", "ascii" }
		end,
	})
	
	-- Random data generation command
	vim.api.nvim_create_user_command("AlchRandom", function(cmd_opts)
		Commands.random(cmd_opts.fargs)
	end, {
		nargs = "*",
		range = true,
		desc = "Generate random data using true randomness",
		complete = function(_, _, _)
			return { "hex", "base64", "base58", "bin", "int", "utf8", "ascii" }
		end,
	})
end

-- Create key mappings (optional, users can set these themselves)
function M.create_keymaps()
	local opts = { noremap = true, silent = true }

	-- Main commands (visual mode and normal mode)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alc",
		"<cmd>AlchConvert<cr>",
		vim.tbl_extend("force", opts, { desc = "🔄 Alchemy: Convert" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>ale",
		"<cmd>AlchExplore<cr>",
		vim.tbl_extend("force", opts, { desc = "🔍 Alchemy: Explore" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alC",
		"<cmd>AlchClassify<cr>",
		vim.tbl_extend("force", opts, { desc = "🎯 Alchemy: Classify" })
	)

	-- Quick conversions with intuitive mnemonics
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alh",
		"<cmd>AlchToHex<cr>",
		vim.tbl_extend("force", opts, { desc = "🔤 Alchemy: To Hex" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>ali",
		"<cmd>AlchToInt<cr>",
		vim.tbl_extend("force", opts, { desc = "🔢 Alchemy: To Int" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>al6",
		"<cmd>AlchToBase64<cr>",
		vim.tbl_extend("force", opts, { desc = "📝 Alchemy: To Base64" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>al5",
		"<cmd>AlchToBase58<cr>",
		vim.tbl_extend("force", opts, { desc = "🔠 Alchemy: To Base58" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alb",
		"<cmd>AlchToBin<cr>",
		vim.tbl_extend("force", opts, { desc = "💾 Alchemy: To Binary" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alu",
		"<cmd>AlchToUtf8<cr>",
		vim.tbl_extend("force", opts, { desc = "📄 Alchemy: To UTF-8" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alA",
		"<cmd>AlchToAscii<cr>",
		vim.tbl_extend("force", opts, { desc = "🔤 Alchemy: To ASCII" })
	)

	-- Hashing operations
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alH",
		"<cmd>AlchHash<cr>",
		vim.tbl_extend("force", opts, { desc = "🔐 Alchemy: Hash" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>al2",
		"<cmd>AlchHash sha256<cr>",
		vim.tbl_extend("force", opts, { desc = "🔐 Alchemy: SHA256" })
	)

	-- Array operations
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alf",
		"<cmd>AlchFlatten<cr>",
		vim.tbl_extend("force", opts, { desc = "📚 Alchemy: Flatten Array" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alk",
		"<cmd>AlchChunk<cr>",
		vim.tbl_extend("force", opts, { desc = "🔪 Alchemy: Chunk Array" })
	)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alr",
		"<cmd>AlchRotate<cr>",
		vim.tbl_extend("force", opts, { desc = "🔄 Alchemy: Rotate Array" })
	)

	-- Generate data
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alg",
		"<cmd>AlchGenerate<cr>",
		vim.tbl_extend("force", opts, { desc = "🎲 Alchemy: Generate" })
	)
	
	-- Generate random data
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alR",
		"<cmd>AlchRandom<cr>",
		vim.tbl_extend("force", opts, { desc = "🎲 Alchemy: Random" })
	)

	-- Telescope integration (if available)
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alt",
		function()
			Telescope.conversion_picker()
		end,
		vim.tbl_extend("force", opts, { desc = "🔭 Alchemy: Telescope Conversions" })
	)
	
	vim.keymap.set(
		{ "n", "v" },
		"<leader>alT",
		function()
			Telescope.classification_picker()
		end,
		vim.tbl_extend("force", opts, { desc = "🔭 Alchemy: Telescope Classifications" })
	)
end

return M
