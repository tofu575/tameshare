import { ListingRequestForm } from "./_components/ListingRequestForm";

export default function Page() {
  return (
    <main>
      <h1>掲載してほしい情報</h1>
      <p>
        試Shareに掲載してほしい方法を見つけたら、出典のURLを送ってください。
      </p>
      <ListingRequestForm />
    </main>
  );
}
