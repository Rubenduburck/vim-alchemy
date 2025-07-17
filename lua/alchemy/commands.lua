-- New modern command interface for alchemy
-- Provides intuitive commands with great UX

local M = {}

local Core = require("alchemy.core")
local UI = require("alchemy.ui")
local Preview = require("alchemy.preview")

-- Main conversion command - smart, works with visual selection or word under cursor
function M.convert(args)
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to convert", vim.log.levels.WARN)
		return
	end

	local output_encoding = args and args[1]

	if not output_encoding then
		-- No output encoding specified, show conversion explorer
		Preview.show_conversion_explorer(text_selection)
		return
	end

	-- Output encoding specified, do direct conversion with preview
	Preview.show_conversion_preview(text_selection, function()
		return Core.classify_and_convert(text_selection.text, output_encoding)
	end, {
		title = " 🔄 Convert to " .. output_encoding .. " ",
	})
end

-- Classification explorer - shows all classifications and conversions
function M.explore()
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to explore", vim.log.levels.WARN)
		return
	end

	local ok, classifications = pcall(Core.classify, text_selection.text)
	if not ok or not classifications then
		UI.notify("Classification failed: " .. (classifications or "unknown error"), vim.log.levels.ERROR)
		return
	end

	Preview.show_classifications(text_selection, classifications)
end

-- Classify command - just show classifications without converting
function M.classify()
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to classify", vim.log.levels.WARN)
		return
	end

	UI.notify("Classifying: " .. text_selection.text, vim.log.levels.INFO)

	local ok, classifications = pcall(Core.classify, text_selection.text)
	if not ok or not classifications then
		UI.notify("Classification failed: " .. (classifications or "unknown error"), vim.log.levels.ERROR)
		return
	end

	Preview.show_classifications(text_selection, classifications)
end

-- Quick conversion commands for common encodings
local function quick_convert_to(encoding)
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to convert", vim.log.levels.WARN)
		return
	end

	Preview.show_conversion_preview(text_selection, function()
		return Core.classify_and_convert(text_selection.text, encoding)
	end, {
		title = " 🎨 Convert to " .. encoding:upper() .. " ",
	})
end

function M.to_hex()
	quick_convert_to("hex")
end

function M.to_int()
	quick_convert_to("int")
end

function M.to_base64()
	quick_convert_to("base64")
end

function M.to_base58()
	quick_convert_to("base58")
end

function M.to_bin()
	quick_convert_to("bin")
end

function M.to_utf8()
	quick_convert_to("utf8")
end

function M.to_ascii()
	quick_convert_to("ascii")
end

function M.to_bytes()
	quick_convert_to("bytes")
end

-- Array manipulation commands (these work with the current text selection)
function M.flatten()
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to flatten", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "array", "flatten", text_selection.text }, false)
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

	local ok, result = pcall(Core.execute_cli, { "array", "chunk", "-c", tostring(chunk_size), text_selection.text }, false)
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

	local ok, result = pcall(Core.execute_cli, { "array", "rotate", "-r", tostring(rotation), text_selection.text }, false)
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Array rotated by %d", rotation))
	else
		vim.notify("Failed to rotate array: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

function M.reverse(args)
	local depth = args and tonumber(args[1]) or 1

	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to reverse", vim.log.levels.WARN)
		return
	end

	local ok, result = pcall(Core.execute_cli, { "array", "reverse", "-d", tostring(depth), text_selection.text }, false)
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Array reversed at depth %d", depth))
	else
		vim.notify("Failed to reverse array: " .. (result or "unknown error"), vim.log.levels.ERROR)
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

	local ok, result = pcall(Core.execute_cli, { "pad", "-s", "left", "-p", tostring(padding), text_selection.text }, false)
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

	local ok, result = pcall(Core.execute_cli, { "pad", "-s", "right", "-p", tostring(padding), text_selection.text }, false)
	if ok and result then
		Core.replace_text(text_selection, result)
		vim.notify(string.format("Padded right to %d", padding))
	else
		vim.notify("Failed to pad: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

-- Hashing commands
function M.hash(args)
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to hash", vim.log.levels.WARN)
		return
	end

	local algorithm = args and args[1]

	if not algorithm then
		local algorithms = {
			{ name = "SHA256", type = "hash", text = "sha256" },
			{ name = "SHA512", type = "hash", text = "sha512" },
			{ name = "SHA3-256", type = "hash", text = "sha3256" },
			{ name = "Blake2-256", type = "hash", text = "blake2256" },
			{ name = "Blake2-512", type = "hash", text = "blake2512" },
			{ name = "Keccak256", type = "hash", text = "keccak256" },
		}
		
		UI.create_selector(algorithms, {
			title = " 🔐 Select Hash Algorithm ",
			on_select = function(selected_algorithm)
				Preview.show_conversion_preview(text_selection, function()
					return Core.execute_cli({ "hash", "-i", "utf8", "-a", selected_algorithm.text, text_selection.text }, false)
				end, {
					title = " 🔐 Hash with " .. selected_algorithm.name .. " ",
				})
			end,
		})
		return
	end

	Preview.show_conversion_preview(text_selection, function()
		return Core.execute_cli({ "hash", "-i", "utf8", "-a", algorithm, text_selection.text }, false)
	end, {
		title = " 🔐 Hash with " .. algorithm:upper() .. " ",
	})
end

-- Generate random data
function M.generate(args)
	local encoding = args and args[1]
	local length = args and tonumber(args[2])

	-- Step 1: Get byte length if not provided
	if not length then
		vim.ui.input({ prompt = "Enter byte length (default: 32): ", default = "32" }, function(input)
			if input then
				local bytes = tonumber(input) or 32
				M.generate({ encoding, tostring(bytes) })
			end
		end)
		return
	end

	-- Step 2: Select encoding if not provided
	if not encoding then
		local encodings = {
			{ name = "Hex", type = "hex", text = "hex" },
			{ name = "Base64", type = "base64", text = "base64" },
			{ name = "Base58", type = "base58", text = "base58" },
			{ name = "Binary", type = "bin", text = "bin" },
			{ name = "Integer", type = "int", text = "int" },
			{ name = "UTF-8", type = "utf8", text = "utf8" },
			{ name = "ASCII", type = "ascii", text = "ascii" },
		}
		
		UI.create_selector(encodings, {
			title = string.format(" 🎲 Generate %d Bytes Data ", length),
			on_select = function(selected_encoding)
				local ok, result = pcall(Core.execute_cli, { "generate", "-e", selected_encoding.text, "-b", tostring(length) }, false)
				if ok and result then
					vim.api.nvim_put({ result }, "c", false, true)
					UI.notify(string.format("Generated %d bytes of random %s data", length, selected_encoding.text), vim.log.levels.INFO)
				else
					UI.notify("Failed to generate: " .. (result or "unknown error"), vim.log.levels.ERROR)
				end
			end,
		})
		return
	end

	-- Step 3: Generate the data
	local ok, result = pcall(Core.execute_cli, { "generate", "-e", encoding, "-b", tostring(length) }, false)
	if ok and result then
		vim.api.nvim_put({ result }, "c", false, true)
		UI.notify(string.format("Generated %d bytes of random %s data", length, encoding), vim.log.levels.INFO)
	else
		UI.notify("Failed to generate: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

-- Generate random data using the random command
function M.random(args)
	local encoding = args and args[1]
	local length = args and tonumber(args[2])

	-- Step 1: Get byte length if not provided
	if not length then
		vim.ui.input({ prompt = "Enter byte length (default: 32): ", default = "32" }, function(input)
			if input then
				local bytes = tonumber(input) or 32
				M.random({ encoding, tostring(bytes) })
			end
		end)
		return
	end

	-- Step 2: Select encoding if not provided
	if not encoding then
		local encodings = {
			{ name = "Hex", type = "hex", text = "hex" },
			{ name = "Base64", type = "base64", text = "base64" },
			{ name = "Base58", type = "base58", text = "base58" },
			{ name = "Binary", type = "bin", text = "bin" },
			{ name = "Integer", type = "int", text = "int" },
			{ name = "UTF-8", type = "utf8", text = "utf8" },
			{ name = "ASCII", type = "ascii", text = "ascii" },
		}
		
		UI.create_selector(encodings, {
			title = string.format(" 🎲 Generate %d Bytes Random Data ", length),
			on_select = function(selected_encoding)
				local ok, result = pcall(Core.execute_cli, { "random", "-e", selected_encoding.text, "-b", tostring(length) }, false)
				if ok and result then
					vim.api.nvim_put({ result }, "c", false, true)
					UI.notify(string.format("Generated %d bytes of random %s data", length, selected_encoding.text), vim.log.levels.INFO)
				else
					UI.notify("Failed to generate: " .. (result or "unknown error"), vim.log.levels.ERROR)
				end
			end,
		})
		return
	end

	-- Step 3: Generate the random data
	local ok, result = pcall(Core.execute_cli, { "random", "-e", encoding, "-b", tostring(length) }, false)
	if ok and result then
		vim.api.nvim_put({ result }, "c", false, true)
		UI.notify(string.format("Generated %d bytes of random %s data", length, encoding), vim.log.levels.INFO)
	else
		UI.notify("Failed to generate: " .. (result or "unknown error"), vim.log.levels.ERROR)
	end
end

return M

