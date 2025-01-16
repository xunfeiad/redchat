<script lang="ts">
  import BackButton from "$lib/components/BackButton.svelte";
  
  const stats = [
    { title: "总用户数", value: "1,234", icon: "👥", color: "#4CAF50" },
    { title: "今日访问", value: "456", icon: "📈", color: "#2196F3" },
    { title: "内容数量", value: "789", icon: "📝", color: "#FF9800" },
    { title: "系统消息", value: "12", icon: "📨", color: "#9C27B0" },
  ];
</script>

<div class="dashboard">
  <div class="header">
    <BackButton />
    <h1>仪表盘</h1>
  </div>
  
  <div class="stats-grid">
    {#each stats as stat}
      <div class="stat-card" style="--card-color: {stat.color}">
        <span class="stat-icon">{stat.icon}</span>
        <div class="stat-info">
          <h3>{stat.title}</h3>
          <p class="stat-value">{stat.value}</p>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .dashboard {
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

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1.5rem;
  }

  .stat-card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    display: flex;
    align-items: center;
    border-left: 4px solid var(--card-color);
    transition: transform 0.2s;
  }

  .stat-card:hover {
    transform: translateY(-2px);
  }

  .stat-icon {
    font-size: 2.5rem;
    margin-right: 1rem;
  }

  .stat-info h3 {
    margin: 0;
    font-size: 1rem;
    color: #666;
  }

  .stat-value {
    margin: 0.5rem 0 0;
    font-size: 1.8rem;
    font-weight: bold;
    color: var(--card-color);
  }
</style>