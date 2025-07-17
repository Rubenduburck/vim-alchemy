-- New modern command interface for alchemy
-- Provides intuitive commands with great UX

local M = {}

local Core = require("alchemy.core")
local Picker = require("alchemy.picker")
local Explorer = require("alchemy.explorer")

-- Main conversion command - smart, works with visual selection or word under cursor
function M.convert(args)
	local output_encoding = args and args[1]

	if not output_encoding then
		-- No output encoding specified, use quick convert with picker
		Explorer.quick_convert()
		return
	end

	-- Output encoding specified, do direct conversion
	Explorer.quick_convert(output_encoding)
end

-- Classification explorer - shows all classifications and conversions
function M.explore()
	Explorer.explore_conversions()
end

-- Classify command - just show classifications without converting
function M.classify()
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to classify", vim.log.levels.WARN)
		return
	end

	vim.notify("Classifying: " .. text_selection.text)

	local ok, classifications = pcall(Core.classify, text_selection.text)
	if not ok or not classifications then
		vim.notify("Classification failed: " .. (classifications or "unknown error"), vim.log.levels.ERROR)
		return
	end

	-- Format and display classifications
	local lines = { "Classifications for: " .. text_selection.text, "" }

	local sorted_classifications = {}
	for encoding, details in pairs(classifications) do
		if type(details) == "table" and details.score ~= nil then
			table.insert(sorted_classifications, {
				encoding = encoding,
				score = details.score,
				details = details,
			})
		end
	end

	table.sort(sorted_classifications, function(a, b)
		return a.score < b.score
	end)

	for _, item in ipairs(sorted_classifications) do
		table.insert(lines, string.format("  %s (score: %d)", item.encoding, item.score))
	end

	-- Show in a floating window or as notifications
	if #lines > 10 then
		-- Use floating window for many results
		local buf = vim.api.nvim_create_buf(false, true)
		vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)

		local width = 0
		for _, line in ipairs(lines) do
			width = math.max(width, #line)
		end

		vim.bo[buf].modifiable = false
		vim.api.nvim_buf_set_keymap(buf, "n", "q", "<cmd>close<cr>", { noremap = true, silent = true })
		vim.api.nvim_buf_set_keymap(buf, "n", "<esc>", "<cmd>close<cr>", { noremap = true, silent = true })
	else
		-- Use notifications for few results
		for i = 3, #lines do -- Skip header lines
			vim.notify(lines[i])
		end
	end
end

-- Quick conversion commands for common encodings
function M.to_hex()
	Explorer.quick_convert("hex")
end

function M.to_int()
	Explorer.quick_convert("int")
end

function M.to_base64()
	Explorer.quick_convert("base64")
end

function M.to_base58()
	Explorer.quick_convert("base58")
end

function M.to_bin()
	Explorer.quick_convert("bin")
end

function M.to_utf8()
	Explorer.quick_convert("utf8")
end

function M.to_ascii()
	Explorer.quick_convert("ascii")
end

function M.to_bytes()
	Explorer.quick_convert("bytes")
end

-- Array manipulation commands (these work with the current text selection)
function M.flatten()
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to flatten", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "flatten-array", text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify("Array flattened")
	else
		vim.notify("Failed to flatten array: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

function M.chunk(args)
	local chunk_size = args and tonumber(args[1])

	if not chunk_size then
		vim.ui.input({ prompt = "Chunk size: " }, function(input)
			if input and tonumber(input) then
				M.chunk({ input })
			end
		end)
		return
	end

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to chunk", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "chunk-array", "-c", tostring(chunk_size), text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Array chunked into groups of %d", chunk_size))
	else
		vim.notify("Failed to chunk array: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

function M.rotate(args)
	local rotation = args and tonumber(args[1])

	if not rotation then
		vim.ui.input({ prompt = "Rotation amount: " }, function(input)
			if input and tonumber(input) then
				M.rotate({ input })
			end
		end)
		return
	end

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to rotate", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "rotate-array", "-r", tostring(rotation), text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Array rotated by %d", rotation))
	else
		vim.notify("Failed to rotate array: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

-- Padding commands
function M.pad_left(args)
	local padding = args and tonumber(args[1])

	if not padding then
		vim.ui.input({ prompt = "Padding amount: " }, function(input)
			if input and tonumber(input) then
				M.pad_left({ input })
			end
		end)
		return
	end

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to pad", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "pad-left", "-p", tostring(padding), text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Padded left to %d", padding))
	else
		vim.notify("Failed to pad: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

function M.pad_right(args)
	local padding = args and tonumber(args[1])

	if not padding then
		vim.ui.input({ prompt = "Padding amount: " }, function(input)
			if input and tonumber(input) then
				M.pad_right({ input })
			end
		end)
		return
	end

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to pad", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "pad-right", "-p", tostring(padding), text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Padded right to %d", padding))
	else
		vim.notify("Failed to pad: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

-- Hashing commands
function M.hash(args)
	local algorithm = args and args[1]

	if not algorithm then
		local algorithms = { "sha256", "sha512", "md5", "blake2", "keccak256" }
		Picker.select_from_list(algorithms, {
			prompt = "Select hash algorithm:",
		}, function(selected_algorithm)
			if selected_algorithm then
				M.hash({ selected_algorithm })
			end
		end)
		return
	end

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to hash", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "classify-and-hash", "-a", algorithm, text_selection.text })
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Hashed with %s", algorithm))
	else
		vim.notify("Failed to hash: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

-- Generate random data
function M.generate(args)
	local encoding = args and args[1]
	local length = args and tonumber(args[2]) or 32

	if not encoding then
		local encodings = { "hex", "base64", "base58", "bin", "int", "utf8", "ascii" }
		Picker.select_from_list(encodings, {
			prompt = "Generate random data in format:",
		}, function(selected_encoding)
			if selected_encoding then
				M.generate({ selected_encoding, tostring(length) })
			end
		end)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "generate", "-e", encoding, "-b", tostring(length) })
	if ok and result then
		vim.api.nvim_put({ result }, "c", false, true)
		vim.notify(string.format("Generated %d bytes of random %s data", length, encoding))
	else
		vim.notify("Failed to generate: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

return M

