local exports = {}

exports.bar = function()
	return "Bar..."
end

exports.health_check = function()
	local health = tonumber(g_var("health"))

	if health < 50 then
		return 0
	else
		return 1
	end
end

return exports
