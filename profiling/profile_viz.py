import re
import pandas as pd
import matplotlib.pyplot as plt
import os
from datetime import datetime
import glob

# Parse log lines
pattern = re.compile(r'System (\w+) took ([\d\.]+)(ns|µs|ms)')
data = []
with open('ecs_profile.log') as f:
    for line in f:
        m = pattern.search(line)
        if m:
            sys, val, unit = m.groups()
            val = float(val)
            # Normalize to microseconds
            if unit == 'ns':
                val = val / 1000
            elif unit == 'ms':
                val = val * 1000
            # else µs, no change
            data.append((sys, val))

df = pd.DataFrame(data, columns=['system', 'time_us'])

# Summary stats
summary = df.groupby('system')['time_us'].agg(['mean', 'std', 'min', 'max', 'count']).sort_values('mean', ascending=False)
print(summary)

# Save summary stats to CSV with timestamp
outdir = "profiling/results"
os.makedirs(outdir, exist_ok=True)
run_id = datetime.now().strftime("%Y%m%d_%H%M%S")
summary_path = os.path.join(outdir, f"profile_summary_{run_id}.csv")
summary.to_csv(summary_path)
print(f"Saved summary to {summary_path}")

# Regression check: compare to previous run if available
csvs = sorted(glob.glob(os.path.join(outdir, "profile_summary_*.csv")))
if len(csvs) > 1:
    prev = pd.read_csv(csvs[-2], index_col=0)
    print("\nRegression Check vs. previous run:")
    for sys in summary.index:
        if sys in prev.index:
            prev_mean = prev.loc[sys, 'mean']
            curr_mean = summary.loc[sys, 'mean']
            change = curr_mean - prev_mean
            pct = 100 * change / prev_mean if prev_mean != 0 else 0
            alert = " <-- REGRESSION" if change > 0 and pct > 5 else ""
            print(f"{sys:35s}: {prev_mean:8.2f} -> {curr_mean:8.2f}  (Δ={change:7.2f} µs, {pct:6.2f}%) {alert}")
else:
    print("No previous run for regression check.")

# Plot histograms for each system (top 4 by mean) on a single figure
top_systems = summary.head(4).index

fig, axes = plt.subplots(2, 2, figsize=(14, 10))
axes = axes.flatten()

for ax, sys in zip(axes, top_systems):
    df[df['system'] == sys]['time_us'].hist(bins=30, alpha=0.7, ax=ax)
    ax.set_title(f"{sys} execution time (µs)")
    ax.set_xlabel("Time (µs)")
    ax.set_ylabel("Count")

# Hide any unused subplots
for i in range(len(top_systems), len(axes)):
    fig.delaxes(axes[i])

plt.tight_layout()
plt.show()

# Time series plots for top systems
for sys in top_systems:
    plt.figure(figsize=(10, 3))
    df[df['system'] == sys].reset_index().plot(y='time_us', use_index=True, title=f"{sys} execution time per tick", legend=False)
    plt.xlabel("Tick")
    plt.ylabel("Time (µs)")
    plt.tight_layout()
    plt.show()