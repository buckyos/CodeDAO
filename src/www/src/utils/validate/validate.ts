
// 检查邮箱
export function validateEmail(email: string): boolean {
    const uPattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return uPattern.test(String(email).toLowerCase());
}

export function validateName(name: string): boolean {
    // const uPattern = /^[a-zA-Z0-9_-]{3,20}$/;
    const uPattern = /^[a-zA-Z0-9]{1}[a-zA-Z0-9_-]{2,19}$/;
    return uPattern.test(name);
}