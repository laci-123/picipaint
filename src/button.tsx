interface Props {
  value: Number,
  onClick: () => void,
}

export function MyButton({value, onClick}: Props) {
  const text = `pressed ${value} times`;
  return <button onClick={onClick}>{text}</button>;
}
