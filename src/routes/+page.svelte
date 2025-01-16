<script lang="ts">
  import Toast from "../lib/components/Toast.svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import type { Response } from "../../types";

  let username = $state("");
  let password = $state("");
  let isLoading = $state(false);

  // Toast 状态
  let showToast = $state(false);
  let toastMessage = $state("");
  let toastType = $state<"success" | "error">("success");

  async function handleLogin(event: Event) {
    // await goto("/chat");
    event.preventDefault();
    isLoading = true;
    const response: Response<object> = await invoke("login", {
      userLogin: { username, password },
    });
    console.log(response);

    if (response.code === 0) {
      localStorage.setItem("isAuthenticated", "true");
      localStorage.setItem("userInfo", JSON.stringify(response.data));
      // 登录成功

      // 显示成功提示
      toastType = "success";
      toastMessage = "登录成功！";
      showToast = true;
      // 延迟跳转
      setTimeout(async () => {
        await goto("/chat");
      }, 1500);
    } else {
      // 登录失败
      showToast = true;
      console.log(response);
      toastType = "error";
      toastMessage = "用户名或密码错误";
      
    }
    isLoading = false;
    console.log(response);
  }
</script>

<Toast show={showToast} type={toastType} message={toastMessage} />

<main class="container">
  <div class="login-box">
    <h1>欢迎登录</h1>

    <form class="login-form" onsubmit={handleLogin}>
      <div class="form-group">
        <label for="username">用户名</label>
        <input
          id="username"
          type="text"
          placeholder="请输入用户名"
          bind:value={username}
          disabled={isLoading}
          required
        />
      </div>

      <div class="form-group">
        <label for="password">密码</label>
        <input
          id="password"
          type="password"
          placeholder="请输入密码"
          bind:value={password}
          disabled={isLoading}
          required
        />
      </div>

      <button type="submit" disabled={isLoading}>
        {#if isLoading}
          登录中...
        {:else}
          登录
        {/if}
      </button>

      <p class="register-link">
        还没有账号？<a href="/register">立即注册</a>
      </p>
    </form>
  </div>
</main>

<style>
  .container {
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #f5f5f5;
  }

  .login-box {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 400px;
  }

  h1 {
    text-align: center;
    color: #333;
    margin-bottom: 2rem;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    color: #666;
    font-size: 0.9rem;
  }

  input {
    padding: 0.8rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }

  input:focus {
    outline: none;
    border-color: #396cd8;
  }

  button {
    background-color: #396cd8;
    color: white;
    padding: 0.8rem;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  button:hover:not(:disabled) {
    background-color: #2857b8;
  }

  button:disabled {
    background-color: #999;
    cursor: not-allowed;
  }

  .register-link {
    text-align: center;
    margin: 0;
    font-size: 0.9rem;
  }

  .register-link a {
    color: #396cd8;
    text-decoration: none;
  }

  .register-link a:hover {
    text-decoration: underline;
  }
</style>
