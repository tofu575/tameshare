import 'package:dummy_gateway/dummy_gateway.dart';
import 'package:model/model.dart';
import 'package:usecase/usecase.dart';

ItemInteractor buildItemInteractor() => ItemInteractor(
  itemGateway: DummyGateway(
    initialItems: <TemplateItem>[
      TemplateItem(
        id: TemplateItemId('first-item'),
        title: TemplateItemTitle('Dummy item'),
      ),
    ],
  ),
);
