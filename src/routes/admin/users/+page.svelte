<script lang="ts">
  import BackButton from "$lib/components/BackButton.svelte";

  // 模拟用户数据
  const users = [
    { id: 1, name: "张三", email: "zhangsan@example.com", role: "管理员", status: "活跃" },
    { id: 2, name: "李四", email: "lisi@example.com", role: "用户", status: "禁用" },
    { id: 3, name: "王五", email: "wangwu@example.com", role: "用户", status: "活跃" },
  ];
</script>

<div class="users-page">
  <div class="header">
    <BackButton />
    <h1>用户管理</h1>
  </div>

  <div class="actions">
    <button class="add-button">
      <span>+</span> 添加用户
    </button>
    <input type="search" placeholder="搜索用户..." class="search-input" />
  </div>

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>用户名</th>
          <th>邮箱</th>
          <th>角色</th>
          <th>状态</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        {#each users as user}
          <tr>
            <td>{user.id}</td>
            <td>{user.name}</td>
            <td>{user.email}</td>
            <td>{user.role}</td>
            <td>
              <span class="status-badge" class:active={user.status === "活跃"}>
                {user.status}
              </span>
            </td>
            <td class="actions-cell">
              <button class="action-btn edit">编辑</button>
              <button class="action-btn delete">删除</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .users-page {
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

  .actions {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }

  .add-button {
    background-color: #4CAF50;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .search-input {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 300px;
  }

  .table-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }

  th {
    background-color: #f5f5f5;
    font-weight: 600;
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.875rem;
    background-color: #ff4444;
    color: white;
  }

  .status-badge.active {
    background-color: #4CAF50;
  }

  .actions-cell {
    display: flex;
    gap: 0.5rem;
  }

  .action-btn {
    padding: 0.25rem 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .action-btn.edit {
    background-color: #2196F3;
    color: white;
  }

  .action-btn.delete {
    background-color: #ff4444;
    color: white;
  }
</style> 