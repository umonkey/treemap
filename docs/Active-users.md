# Active Users

We don't specifically track active users, but we track changes to tree properties.
Each change has a timestamp and author, so we can determine who has made changes and when.

To get DAU from the SQLite database, use the following query:

```sql
SELECT strftime('%Y-%m-%d', added_at, 'unixepoch') AS date, COUNT(DISTINCT added_by) FROM trees_props GROUP BY date ORDER BY date DESC LIMIT 30;
```

To get MAU from the SQLite database, use the following query:

```sql
SELECT strftime('%Y-%m-01', added_at, 'unixepoch') AS date, COUNT(DISTINCT added_by) FROM trees_props GROUP BY date ORDER BY date DESC LIMIT 30;
```

Active users for last month:

```sql
SELECT COUNT(DISTINCT added_by) FROM trees_props WHERE added_at >= CURRENT_TIMESTAMP - 86400 * 30;
```

Active users for last week:

```sql
SELECT COUNT(DISTINCT added_by) FROM trees_props WHERE added_at >= CURRENT_TIMESTAMP - 86400 * 7;
```
