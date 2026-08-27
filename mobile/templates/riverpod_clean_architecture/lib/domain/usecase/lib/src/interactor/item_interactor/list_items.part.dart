part of '../item_interactor.dart';

Future<List<TemplateItem>> _listItems(ItemInteractor interactor) =>
    interactor._itemGateway.findAll();
