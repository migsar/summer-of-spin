import { Kv, ResponseBuilder } from "@fermyon/spin-sdk";

export async function handler(req: Request, res: ResponseBuilder) {
    const { url } = req;
    const store = Kv.openDefault();
    const { pathname } = new URL(url);
    const tag = pathname.slice(1);
    const itineraryBytes = store.get(tag);

    if (itineraryBytes) {
        const itinerary = new TextDecoder().decode(itineraryBytes);

        res.status(200)
          .set({ "content-type": "application/json" })
          .send(JSON.stringify({ tag, itinerary }));
    } else {
        res.status(404).send('Not found!');
    }
}
