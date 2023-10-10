fn display_neutron_flow(polarity: isize) {
  println!(
    "Neutron Flow is {}",
    if polarity < 0 { "reversed" } else { "normal" }
  );
}

pub fn call() {
  let polarity = 1;
  {
    let polarity = polarity - 2;
    display_neutron_flow(polarity);
  }
  display_neutron_flow(polarity);
}
