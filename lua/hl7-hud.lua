local augroup = vim.api.nvim_create_augroup("hl7-hud", { clear = true })
local path_to_hl7_hud = "hl7-hud"

local function setup(opts)
    -- load the path to the hl7-hud binary
    if opts and opts.path then
        path_to_hl7_hud = opts.path
    end
    
    -- detect hl7 file types
    vim.api.nvim_create_autocmd({"BufRead", "BufNewFile"},
        { group = augroup, desc = "Set filetype to hl7", pattern = "*.hl7", callback = function()
            vim.bo.filetype = "hl7"
        end })
end

local buffer_to_string = function()
    local content = vim.api.nvim_buf_get_lines(0, 0, vim.api.nvim_buf_line_count(0), false)
    return table.concat(content, "\r")
end

local function hl7_cursor_pos()
    cursor_pos = vim.fn.getcurpos()
    buffer_offset = vim.fn.line2byte(cursor_pos[2]) + cursor_pos[3] - 1

    -- execute the hl7 hud binary
    local output = vim.system({path_to_hl7_hud, "p", buffer_offset}, {
        text = true,
        stdin = buffer_to_string(),
    }):wait()

    if output.code == 0 then
        return vim.fn.trim(output.stdout)
    else
        return "Error: " .. vim.fn.trim(output.stderr)
    end
end

local function hl7_query(query)
    -- execute the hl7 hud binary
    local output = vim.system({path_to_hl7_hud, "q", query}, {
        text = true,
        stdin = buffer_to_string(),
    }):wait()

    if output.code == 0 then
        local pos = tonumber(vim.fn.trim(output.stdout))
        if pos >= 0 then
            -- convert the byte offset to a line number and column
            local line = vim.fn.byte2line(pos + 1)
            local col = pos - vim.fn.line2byte(line)
            if col < 0 then
                col = 0
            end

            vim.api.nvim_win_set_cursor(0, {line, col})
        end
    else
        print("Error: " .. vim.fn.trim(output.stderr))
    end
end

local function hl7_query_input()
    local query = vim.fn.input("HL7 Query: ")
    if query ~= "" then
        hl7_query(query)
    end
end

local lualine_ext = {}
lualine_ext.sections = {
    lualine_a = {'mode'},
    lualine_b = {'branch', 'diff', 'diagnostics'},
    lualine_c = {'filename'},
    lualine_x = {'encoding', 'fileformat', 'filetype'},
    lualine_y = {'progress'},
    lualine_z = {'location', hl7_cursor_pos},
}
lualine_ext.filetypes = { "hl7" }

return { setup = setup, hl7_cursor_pos = hl7_cursor_pos, hl7_query = hl7_query, hl7_query_input = hl7_query_input, lualine_ext = lualine_ext }
