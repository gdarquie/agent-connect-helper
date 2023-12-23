
function check() {
  db = db.getSiblingDB("core-fca-low");

  const fqdns = db.fqdnToProvider.find();
  let result = [];

  fqdns.forEach(fqdn => {
    const provider = db.provider.findOne({'uid': fqdn.identityProvider});
    if (provider) {
      result.push({uid: provider.uid, name: provider.name, fqdn: fqdn.fqdn})
    }
  });

  print(JSON.stringify(result));
}

check();