<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import type { Response } from "../../types";

  let formData = {
    username: "",
    password: "",
    confirmPassword: "",
    email: "",
    verifyCode: "",
  };

  let errorMessage = "";
  let isLoading = false;
  let isSendingCode = false;
  let countdown = 0;
  let timer: number;

  async function sendVerifyCode() {
    if (!formData.email) {
      errorMessage = "请输入邮箱";
      return;
    }
    
    // 添加邮箱格式校验
    const emailRegex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/;
    if (!emailRegex.test(formData.email)) {
      errorMessage = "请输入正确的邮箱格式";
      return;
    }

    try {
      isSendingCode = true;
      const response: Response = await invoke("send_email_code", { email: formData.email });
      console.log(response);
      
      // 开始倒计时
      countdown = 60;
      timer = setInterval(() => {
        countdown--;
        if (countdown <= 0) {
          clearInterval(timer);
        }
      }, 1000);

    } catch (error) {
      console.log(error);
      errorMessage = `发送验证码失败: ${error}`;
    } finally {
      isSendingCode = false;
    }
  }

  async function handleRegister(event: Event) {
    event.preventDefault();
    
    if (formData.password !== formData.confirmPassword) {
      errorMessage = "两次密码输入不一致";
      return;
    }

    if (!formData.verifyCode) {
      errorMessage = "请输入验证码";
      return;
    }

    isLoading = true;
    errorMessage = "";

    const { confirmPassword, verifyCode, ...registerData } = formData;
      let response: Response = await invoke("register", { userRegister: { username: registerData.username, password: registerData.password, email: registerData.email, code: verifyCode } });
      if (response.code === 0) {
        await goto("/");
      } else {
        errorMessage = `注册失败: ${response.message}`;
      }
    isLoading = false;
  }

  // 组件卸载时清理定时器
  import { onDestroy } from 'svelte';
  onDestroy(() => {
    if (timer) clearInterval(timer);
  });
</script>

<div class="register-container">
  <form on:submit={handleRegister} class="register-form">
    <h1>注册账号</h1>

    {#if errorMessage}
      <div class="error-message">{errorMessage}</div>
    {/if}

    <div class="form-group">
      <label for="username">用户名 *</label>
      <input
        type="text"
        id="username"
        bind:value={formData.username}
        required
        minlength="3"
        maxlength="20"
        placeholder="请输入用户名"
      />
    </div>

    <div class="form-group">
      <label for="password">密码 *</label>
      <input
        type="password"
        id="password"
        bind:value={formData.password}
        required
        minlength="6"
        placeholder="请输入密码"
      />
    </div>

    <div class="form-group">
      <label for="confirmPassword">确认密码 *</label>
      <input
        type="password"
        id="confirmPassword"
        bind:value={formData.confirmPassword}
        required
        placeholder="请再次输入密码"
      />
    </div>

    <div class="form-group">
      <label for="email">邮箱 *</label>
      <input
        type="email"
        id="email"
        bind:value={formData.email}
        required
        placeholder="请输入邮箱"
      />
    </div>

    <div class="form-group">
      <label for="verifyCode">验证码 *</label>
      <div class="verify-code-group">
        <input
          type="text"
          id="verifyCode"
          bind:value={formData.verifyCode}
          required
          placeholder="请输入验证码"
        />
        <button 
          type="button" 
          class="send-code-btn"
          on:click={sendVerifyCode}
          disabled={countdown > 0 || isSendingCode || !formData.email}
        >
          {#if countdown > 0}
            {countdown}s
          {:else if isSendingCode}
            发送中
          {:else}
            发送
          {/if}
        </button>
      </div>
    </div>

    <button type="submit" disabled={isLoading}>
      {isLoading ? '注册中...' : '注册'}
    </button>

    <div class="login-link">
      已有账号？ <a href="/">立即登录</a>
    </div>
  </form>
</div>

<style>
  :root {
    font-size: clamp(14px, 1vw, 16px);
  }

  .register-container {
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: clamp(1rem, 2vw, 2rem);
    background: #f5f5f5;
  }

  .register-form {
    background: white;
    padding: clamp(1.5rem, 3vw, 2rem);
    border-radius: 0.75rem;
    box-shadow: 0 0.25rem 0.375rem rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 25rem;
  }

  h1 {
    text-align: center;
    color: #333;
    margin-bottom: 2rem;
    font-size: clamp(1.5rem, 3vw, 1.75rem);
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #555;
    font-weight: 500;
    font-size: 0.9rem;
  }

  input, button {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 0.375rem;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .verify-code-group {
    display: flex;
    gap: 0.5rem;
  }

  .verify-code-group input {
    flex: 0.7;
  }

  .send-code-btn {
    flex: 0.3;
    white-space: nowrap;
    padding: 0 1rem;
    background: #4CAF50;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  button[type="submit"] {
    background: #1976D2;
    color: white;
    font-weight: 600;
    font-size: 1rem;
    padding: clamp(0.75rem, 1.5vw, 1rem);
  }

  .error-message {
    background: #ffebee;
    color: #c62828;
    padding: 0.75rem;
    border-radius: 0.375rem;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
  }

  .login-link {
    text-align: center;
    margin-top: 1.5rem;
    color: #666;
    font-size: 0.875rem;
  }

  @media (max-width: 480px) {
    :root {
      font-size: 14px;
    }

    .register-form {
      padding: 1.25rem;
    }

    .verify-code-group {
      flex-direction: row;
    }

    .send-code-btn {
      width: 30%;
      padding: 0.75rem 0;
    }
  }

  @media (min-width: 1200px) {
    :root {
      font-size: 16px;
    }
  }
</style> 