import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator

# Example data
data = {
    'vk': [0.1315109, 0.0046332, 0.0052506, 0.0049473, 0.0056131, 0.005289889],   # [max, min, median, 5%, 95%, avg]
    'vk np': [0.2317643, 0.0056508, 0.0063748, 0.0060236, 0.0067794, 0.0064345906],
    'dx12': [0.1522989, 0.0045233, 0.0051079, 0.0047925, 0.005501, 0.005156559],
    'dx12 np': [0.1456134, 0.0056593, 0.0062483, 0.0059291, 0.0067067, 0.006306328],
    'gl': [0.21306351, 0.0111363, 0.0120061, 0.0115117, 0.0132547, 0.012219194],
    'gl np': [0.1935744, 0.0119844, 0.0128479, 0.0123867, 0.0139332, 0.0130567495],
}

labels = list(data.keys())
positions = range(len(labels))

fig, ax = plt.subplots(dpi=250)

for i, key in enumerate(labels):
    maximum, minimum, median, p5, p95, avg = list(x*1000 for x in data[key])

    # Draw box between 5th and 95th percentile
    ax.add_patch(plt.Rectangle((i - 0.2, p5), 0.4, p95 - p5, edgecolor='black', facecolor='lightblue'))

    # Draw median line
    ax.plot([i - 0.2, i + 0.2], [median, median], color='red', label="Medián" if i == 0 else "", linewidth=2)

    # Optional: mark the mean
    ax.plot(i, avg, marker='o', color='green', label='Průměr' if i == 0 else "")

ax.set_xticks(positions)
ax.set_xticklabels(labels)
ax.yaxis.set_major_locator(MaxNLocator(nbins=16))
ax.set_ylabel('Čas snímku (ms)')
ax.set_xlabel('Parametry')
ax.legend()
plt.grid(True, axis='y', linestyle='--', alpha=0.7)
plt.tight_layout()
plt.show()
