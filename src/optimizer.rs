pub fn optimize(mut code: String) -> String {
    while code.contains("><") || code.contains("<>") ||
        code.contains("+-") || code.contains("-+") {
        code = code.replace("><", "");
        code = code.replace("<>", "");
        code = code.replace("+-", "");
        code = code.replace("-+", "");
    }
    code
}