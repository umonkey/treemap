# Statistics

This document describes how to get statistics from the SQLite database.


## Monthly active users

Historic data, e.g. number per month:

```sql
SELECT strftime('%Y-%m-01', added_at, 'unixepoch') AS date, COUNT(DISTINCT added_by) AS mau FROM trees_props GROUP BY date ORDER BY date DESC LIMIT 12;
```

Current month:

```sql
SELECT COUNT(DISTINCT added_by) AS mau FROM trees_props WHERE added_at >= strftime('%s', 'now') - 86400 * 30;
```

## Daily active users

Historic data, e.g. number per day:

```sql
SELECT strftime('%Y-%m-%d', added_at, 'unixepoch') AS date, COUNT(DISTINCT added_by) AS dau FROM trees_props GROUP BY date ORDER BY date DESC LIMIT 30;
```


## Last week active users

```sql
SELECT COUNT(DISTINCT added_by) AS wau FROM trees_props WHERE added_at >= strftime('%s', 'now') - 86400 * 7;
```


## Trees added last week

```sql
SELECT COUNT(1) FROM trees WHERE added_at >= strftime('%s', 'now') - 86400 * 7;
```


## Trees updated last week

```sql
SELECT COUNT(1) FROM trees WHERE updated_at >= strftime('%s', 'now') - 86400 * 7;
```


## Most active users last week

```sql
SELECT t2.id, t2.name, COUNT(1) AS c FROM trees_props t1 INNER JOIN users t2 ON t2.id = t1.added_by WHERE added_at >= strftime('%s', 'now') - 86400 * 7 GROUP BY t1.added_by ORDER BY c DESC;
```
