<script lang="ts">
  import BackButton from "$lib/components/BackButton.svelte";

  let settings = {
    siteName: "我的管理系统",
    siteDescription: "一个强大的后台管理系统",
    enableNotifications: true,
    theme: "light",
    language: "zh_CN"
  };

  function handleSubmit() {
    // 处理设置保存逻辑
    alert("设置已保存");
  }
</script>

<div class="settings-page">
  <div class="header">
    <BackButton />
    <h1>系统设置</h1>
  </div>

  <div class="settings-container">
    <form on:submit|preventDefault={handleSubmit}>
      <div class="setting-group">
        <h2>基本设置</h2>
        
        <div class="form-group">
          <label for="siteName">网站名称</label>
          <input 
            id="siteName"
            type="text" 
            bind:value={settings.siteName}
          />
        </div>

        <div class="form-group">
          <label for="siteDescription">网站描述</label>
          <textarea 
            id="siteDescription"
            bind:value={settings.siteDescription}
          ></textarea>
        </div>
      </div>

      <div class="setting-group">
        <h2>界面设置</h2>
        
        <div class="form-group">
          <label for="theme">主题</label>
          <select id="theme" bind:value={settings.theme}>
            <option value="light">浅色</option>
            <option value="dark">深色</option>
          </select>
        </div>

        <div class="form-group">
          <label for="language">语言</label>
          <select id="language" bind:value={settings.language}>
            <option value="zh_CN">简体中文</option>
            <option value="en_US">English</option>
          </select>
        </div>

        <div class="form-group checkbox">
          <label>
            <input 
              type="checkbox" 
              bind:checked={settings.enableNotifications}
            />
            启用通知
          </label>
        </div>
      </div>

      <div class="form-actions">
        <button type="submit" class="save-button">保存设置</button>
      </div>
    </form>
  </div>
</div>

<style>
  .settings-page {
    padding: 1rem;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
  }

  .settings-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    padding: 2rem;
  }

  .setting-group {
    margin-bottom: 2rem;
  }

  .setting-group h2 {
    font-size: 1.2rem;
    color: #333;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid #eee;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #666;
  }

  input[type="text"],
  textarea,
  select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }

  textarea {
    height: 100px;
    resize: vertical;
  }

  .checkbox {
    display: flex;
    align-items: center;
  }

  .checkbox label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
  }

  .form-actions {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid #eee;
  }

  .save-button {
    background-color: #4CAF50;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  .save-button:hover {
    background-color: #45a049;
  }
</style> 