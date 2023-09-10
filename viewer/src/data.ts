export function osmUrl(props: { [name: string]: string }): string {
  if (props.node_id) {
    return `http://openstreetmap.org/node/${props.node_id}`;
  }
  if (props.way_id) {
    return `http://openstreetmap.org/way/${props.way_id}`;
  }
  if (props.relation_id) {
    return `http://openstreetmap.org/relation/${props.relation_id}`;
  }
  throw new Error("Input props not an OSM object at all");
}
