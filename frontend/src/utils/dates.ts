export const formatDate = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);

  const day = ("00" + date.getDate()).substr(-2);
  const month = ("00" + (date.getMonth() + 1)).substr(-2);
  const year = date.getFullYear();

  return `${day}.${month}.${year}`;
}
