#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
SHA-256 双向链 + 三重指纹 + 聚合加密密码生成器
命令行增强版 - 精美界面
"""

import struct
import sys
import os
import getpass
import time

# ============================================================
# 颜色定义
# ============================================================
COLOR_CYAN = '\033[96m'
COLOR_PURPLE = '\033[95m'
COLOR_GOLD = '\033[93m'
COLOR_RED = '\033[91m'
COLOR_GREEN = '\033[92m'
COLOR_BLUE = '\033[94m'
COLOR_RESET = '\033[0m'
COLOR_BOLD = '\033[1m'
COLOR_DIM = '\033[2m'

# ============================================================
# 界面组件
# ============================================================
def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def print_banner():
    banner = f"""
{COLOR_CYAN}{COLOR_BOLD}╔══════════════════════════════════════════════════════════════════╗
║                                                                  ║
║   ███████╗██╗  ██╗ █████╗     ██████╗ ██╗███╗   ██╗            ║
║   ██╔════╝██║  ██║██╔══██╗    ██╔══██╗██║████╗  ██║            ║
║   ███████╗███████║███████║    ██████╔╝██║██╔██╗ ██║            ║
║   ╚════██║██╔══██║██╔══██║    ██╔═══╝ ██║██║╚██╗██║            ║
║   ███████║██║  ██║██║  ██║    ██║     ██║██║ ╚████║            ║
║   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝    ╚═╝     ╚═╝╚═╝  ╚═══╝            ║
║                                                                  ║
║           SHA-256 双向链 · 三重指纹 · 聚合加密                  ║
║                   双输入增强版 · 密码生成器                      ║
╚══════════════════════════════════════════════════════════════════╝{COLOR_RESET}
"""
    print(banner)

def print_section(title, color=COLOR_BLUE):
    print(f"\n{color}{'─' * 70}{COLOR_RESET}")
    print(f"{color}{COLOR_BOLD}  {title}{COLOR_RESET}")
    print(f"{color}{'─' * 70}{COLOR_RESET}")

def print_success(text):
    print(f"{COLOR_GREEN}✅ {text}{COLOR_RESET}")

def print_error(text):
    print(f"{COLOR_RED}❌ {text}{COLOR_RESET}")

def print_info(text):
    print(f"{COLOR_CYAN}ℹ️  {text}{COLOR_RESET}")

def print_result(text):
    print(f"{COLOR_GOLD}🔑 {text}{COLOR_RESET}")

def print_waiting():
    print(f"\n{COLOR_DIM}按回车键退出程序...{COLOR_RESET}", end='', flush=True)
    input()

def loading_animation():
    chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']
    for i in range(10):
        sys.stdout.write(f'\r{COLOR_PURPLE}  {chars[i % len(chars)]} 正在计算哈希...{COLOR_RESET}')
        sys.stdout.flush()
        time.sleep(0.05)
    sys.stdout.write('\r' + ' ' * 30 + '\r')

# ============================================================
# SHA-256 核心
# ============================================================
H0 = 0x6a09e667
H1 = 0xbb67ae85
H2 = 0x3c6ef372
H3 = 0xa54ff53a
H4 = 0x510e527f
H5 = 0x9b05688c
H6 = 0x1f83d9ab
H7 = 0x5be0cd19

K = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
]

def rotr(x, n):
    return ((x >> n) | (x << (32 - n))) & 0xffffffff

def shr(x, n):
    return x >> n

def sigma0(x):
    return rotr(x, 7) ^ rotr(x, 18) ^ shr(x, 3)

def sigma1(x):
    return rotr(x, 17) ^ rotr(x, 19) ^ shr(x, 10)

def Sigma0(x):
    return rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)

def Sigma1(x):
    return rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)

def ch(x, y, z):
    return (x & y) ^ (~x & z)

def maj(x, y, z):
    return (x & y) ^ (x & z) ^ (y & z)

def pad_message(msg_bytes):
    orig_len = len(msg_bytes) * 8
    msg_bytes += b'\x80'
    while (len(msg_bytes) * 8) % 512 != 448:
        msg_bytes += b'\x00'
    msg_bytes += struct.pack('>Q', orig_len)
    return msg_bytes

def sha256(msg_bytes):
    padded = pad_message(msg_bytes)
    h0, h1, h2, h3, h4, h5, h6, h7 = H0, H1, H2, H3, H4, H5, H6, H7

    for chunk_idx in range(0, len(padded), 64):
        chunk = padded[chunk_idx:chunk_idx+64]
        w = list(struct.unpack('>16I', chunk))

        for i in range(16, 64):
            w.append((sigma1(w[i-2]) + w[i-7] + sigma0(w[i-15]) + w[i-16]) & 0xffffffff)

        a, b, c, d, e, f, g, h = h0, h1, h2, h3, h4, h5, h6, h7

        for i in range(64):
            temp1 = (h + Sigma1(e) + ch(e, f, g) + K[i] + w[i]) & 0xffffffff
            temp2 = (Sigma0(a) + maj(a, b, c)) & 0xffffffff
            h = g
            g = f
            f = e
            e = (d + temp1) & 0xffffffff
            d = c
            c = b
            b = a
            a = (temp1 + temp2) & 0xffffffff

        h0 = (h0 + a) & 0xffffffff
        h1 = (h1 + b) & 0xffffffff
        h2 = (h2 + c) & 0xffffffff
        h3 = (h3 + d) & 0xffffffff
        h4 = (h4 + e) & 0xffffffff
        h5 = (h5 + f) & 0xffffffff
        h6 = (h6 + g) & 0xffffffff
        h7 = (h7 + h) & 0xffffffff

    digest = struct.pack('>8I', h0, h1, h2, h3, h4, h5, h6, h7)
    return digest.hex()

def extract_digits(hash_str):
    return [c for c in hash_str if c.isdigit()]

def generate_password(digits_list, start_pos, password_len=6):
    if not digits_list or len(digits_list) <= start_pos:
        return ""
    n = len(digits_list)
    password = []
    pos = start_pos % n
    for _ in range(password_len):
        current_digit = int(digits_list[pos])
        password.append(str(current_digit))
        step = current_digit + 1
        pos = (pos + step) % n
    return ''.join(password)

def run_chain(input_str, chain_name="链"):
    print_section(f"{chain_name}", COLOR_CYAN)
    print(f"  输入: {COLOR_DIM}{input_str[:20]}...{COLOR_RESET}")
    
    loading_animation()
    
    hash1 = sha256(input_str.encode('utf-8'))
    digits1 = extract_digits(hash1)
    if len(digits1) < 3:
        print_error(f"{chain_name} 第1轮数字太少！")
        return None, None, None
    A = generate_password(digits1, start_pos=2, password_len=6)
    print_info(f"第1轮 → A = {COLOR_GOLD}{A}{COLOR_RESET}")
    
    loading_animation()
    
    hash2 = sha256(A.encode('utf-8'))
    digits2 = extract_digits(hash2)
    if len(digits2) < 6:
        print_error(f"{chain_name} 第2轮数字太少！")
        return None, None, None
    B = generate_password(digits2, start_pos=5, password_len=6)
    print_info(f"第2轮 → B = {COLOR_GOLD}{B}{COLOR_RESET}")
    
    loading_animation()
    
    combined = A + B
    hash3 = sha256(combined.encode('utf-8'))
    digits3 = extract_digits(hash3)
    if len(digits3) < 4:
        print_error(f"{chain_name} 第3轮数字太少！")
        return None, None, None
    result = generate_password(digits3, start_pos=3, password_len=6)
    print_success(f"第3轮 → {chain_name}结果 = {COLOR_GOLD}{result}{COLOR_RESET}")

    return A, B, result

# ============================================================
# 主程序
# ============================================================
def main():
    clear_screen()
    print_banner()
    
    print_section("安全提示", COLOR_GOLD)
    print(f"""
  {COLOR_CYAN}🔒 输入1:{COLOR_RESET} 网站/账号标识（可公开，如生日、邮箱）
  {COLOR_RED}🔐 输入2:{COLOR_RESET} 你的主密码/盐值（{COLOR_BOLD}只有你知道，请保密！{COLOR_RESET}）
  {COLOR_PURPLE}⚠️  {COLOR_RESET}两个输入组合才能生成最终密码，{COLOR_BOLD}缺一不可{COLOR_RESET}
  {COLOR_DIM}💡 按 Ctrl+C 可随时退出程序{COLOR_RESET}
""")
    
    try:
        # 输入1
        print_section("输入信息", COLOR_GREEN)
        input1 = input(f"{COLOR_GREEN}  📌 网站/账号标识: {COLOR_RESET}")
        if not input1.strip():
            print_error("输入不能为空！")
            print_waiting()
            return
        
        # 输入2
        input2 = getpass.getpass(f"{COLOR_RED}  🔐 主密码/盐值 (不会显示): {COLOR_RESET}")
        if not input2.strip():
            print_error("主密码不能为空！")
            print_waiting()
            return
        
        # 确认
        input2_confirm = getpass.getpass(f"{COLOR_RED}  🔐 再次输入主密码确认: {COLOR_RESET}")
        if input2 != input2_confirm:
            print_error("两次输入的主密码不一致！")
            print_waiting()
            return

    except KeyboardInterrupt:
        print(f"\n\n{COLOR_GOLD}👋 用户取消，已退出。{COLOR_RESET}")
        print_waiting()
        return
    except Exception as e:
        print_error(f"输入错误: {e}")
        print_waiting()
        return

    # 混合
    print_section("混合种子", COLOR_PURPLE)
    max_len = max(len(input1), len(input2))
    mixed_chars = []
    for i in range(max_len):
        if i < len(input1):
            mixed_chars.append(input1[i])
        if i < len(input2):
            mixed_chars.append(input2[i])
    mixed_input = ''.join(mixed_chars)
    
    loading_animation()
    seed_hash = sha256(mixed_input.encode('utf-8'))
    print_info(f"混合种子: {COLOR_DIM}{seed_hash[:16]}... (隐藏){COLOR_RESET}")
    
    final_seed = seed_hash

    print(f"\n  {COLOR_GREEN}📌 账号标识:{COLOR_RESET} {input1}")
    print(f"  {COLOR_RED}🔐 主密码:{COLOR_RESET} {'*' * len(input2)} (已隐藏)")

    # 执行双向链
    forward_input = final_seed
    reverse_input = final_seed[::-1]

    fA, fB, f_result = run_chain(forward_input, "正向链")
    if f_result is None:
        print_error("正向链生成失败！")
        print_waiting()
        return

    rA, rB, r_result = run_chain(reverse_input, "反向链")
    if r_result is None:
        print_error("反向链生成失败！")
        print_waiting()
        return

    # 最终聚合
    print_section("最终结果", COLOR_RED)
    
    combined_final = f_result + r_result
    print_info(f"正向 + 反向拼接: {COLOR_GOLD}{combined_final}{COLOR_RESET}")
    
    loading_animation()
    hash_final = sha256(combined_final.encode('utf-8'))
    print_info(f"最终 SHA-256: {COLOR_DIM}{hash_final}{COLOR_RESET}")

    digits_final = extract_digits(hash_final)
    print_result(f"提取数字: {COLOR_GOLD}{''.join(digits_final)}{COLOR_RESET}")

    if len(digits_final) < 3:
        print_error("最终数字太少！")
        print_waiting()
        return

    final_password = generate_password(digits_final, start_pos=2, password_len=6)
    
    # 最终密码 - 大号显示
    print(f"\n{COLOR_RED}{COLOR_BOLD}{'═' * 70}{COLOR_RESET}")
    print(f"{COLOR_RED}{COLOR_BOLD}  🎯 最终密码: {final_password}{COLOR_RESET}")
    print(f"{COLOR_RED}{COLOR_BOLD}{'═' * 70}{COLOR_RESET}")

    print_section("安全提醒", COLOR_GOLD)
    print(f"""
  {COLOR_GREEN}✅{COLOR_RESET} 请牢记你的主密码，丢失无法找回！
  {COLOR_GREEN}✅{COLOR_RESET} 不同网站建议使用不同的标识（输入1）
  {COLOR_RED}⚠️  {COLOR_RESET} 不要将主密码告诉任何人
  {COLOR_DIM}💡 按 Ctrl+C 可随时退出{COLOR_RESET}
""")
    
    print_waiting()

if __name__ == '__main__':
    main()