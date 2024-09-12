class_name RivetLogger

static func log(args):
	print("[Rivet] ", args)

static func warning(args):
	print("[Rivet] ", args)
	push_warning("[Rivet] ", args)

static func error(args):
	print("[Rivet] ", args)
	push_error("[Rivet] ", args)

