const data = [
  {
    fqdn: "montest.fr",
    identityProvider: "9c716f61-b8a1-435c-a407-ef4d677ec270",
  },
];

// check fqdn respects the correct format (sub.)domain.tld
function isValidFqdn(fqdn) {
  print(`Test "${fqdn}" fqdn.`);
  const fqdnRegex = /^([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}$/;
  return fqdnRegex.test(fqdn);
}

// {fqdn: (sub.)domain.tld, idp: uuid}
function addFqdn(fqdnToProvider, isDryRun) {
  if (!isValidFqdn(fqdnToProvider.fqdn)) {
    print(`"${fqdnToProvider.fqdn}" is not a valid fqdn.`);
    return;
  }
  print(`"${fqdnToProvider.fqdn}" is a valid fqdn.`);
  false;
  const provider = db.provider.findOne({
    uid: fqdnToProvider.identityProvider,
  });

  if (!provider) {
    print(`No provider found for uuid "${fqdnToProvider.identityProvider}".`);
    return;
  }

  // check if already exists
  const isExisting = db.fqdnToProvider.findOne({
    identityProvider: fqdnToProvider.identityProvider,
    fqdn: fqdnToProvider.fqdn,
  });

  if (isExisting) {
    print(
      `This configuration already exists: idp is "${fqdnToProvider.identityProvider}" and fqdn "${fqdnToProvider.fqdn}". No data has been changed.`
    );
    return;
  }

  if (provider && !isDryRun) {
    db.fqdnToProvider.insert({
      fqdn: fqdnToProvider.fqdn,
      identityProvider: fqdnToProvider.identityProvider,
    });

    print(
      `Update successfully completes for fqdn "${fqdnToProvider.fqdn}" and idp "${fqdnToProvider.identityProvider}".`
    );
  }

  if (isDryRun) {
    print(
      `Dry run successfully simulates adding conf for fqdn "${fqdnToProvider.fqdn}" and idp "${fqdnToProvider.identityProvider}". No data has been changed.`
    );
  }
}

function removeFqdn(fqdnToProvider, isDryRun) {
  if (!isDryRun) {
    db.fqdnToProvider.remove({
      fqdn: fqdnToProvider.fqdn,
      identityProvider: fqdnToProvider.identityProvider,
    });
    print(
      `Rollback successfully completes for fqdn "${fqdnToProvider.fqdn}" and for idp "${fqdnToProvider.identityProvider}".`
    );
  }
  if (isDryRun) {
    print(
      `Dry run successfully simulates the rollback for fqdn "${fqdnToProvider.fqdn}" and idp "${fqdnToProvider.identityProvider}". No data has been changed.`
    );
  }
}

// [fqdn: string, idp: uuid]
function updateFqdns(fqdnToProviders, isDryRun = true, isRollback = false) {
  print(`------------------------`);
  print(`Migration starts.`);
  print(`DRY RUN: ${isDryRun}.`);
  print(`ROLLBACK: ${isRollback}.`);
  print(`------------------------`);

  fqdnToProviders.forEach((fqdnToProvider) => {
    if (!isRollback) {
      addFqdn(fqdnToProvider, isDryRun);
    } else {
      removeFqdn(fqdnToProvider, isDryRun);
    }
  });
}

updateFqdns(data, dryrun, rollback);
