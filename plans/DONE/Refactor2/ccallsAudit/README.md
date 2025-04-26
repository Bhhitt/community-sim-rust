# How to View Graphviz ECS System Graphs

To view a system's call/resource access graph, run the following command (replace the `.dot` filename if needed):

```sh
dot -Tpdf collect_food_positions_system.dot -o collect_food_positions_system.pdf && open collect_food_positions_system.pdf
```

This will generate a PDF and open it for you on macOS. You can use any other Graphviz-supported output format (e.g., png, svg) by changing `-Tpdf`.

---

- Each `.dot` file in this directory corresponds to a single ECS system's call/resource graph.
- You can edit or extend these graphs as you audit more systems.
