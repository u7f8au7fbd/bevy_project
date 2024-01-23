
# Bevyのテンプレート

> [!NOTE]
> Bevyでdynamic_linkingを有効にしたプロジェクトのテンプレートです.<br>
> ※Rustのnightly版をインストールしている必要があります.<br>

> [!IMPORTANT]
> launch.json内部の "filter"の"name"とCargo.tomlのnameは手動で設定する必要があり,<br>
> ワークスペースの名前は同じ名前である必要があります.<br>

>[!TIP]
> Provatheusプラグインは起動時のウィンドウサイズから自動的にグリッドを生成します.<br>
>   2DカメラのZ軸を変更すると正しい範囲での描画がされません.<br>
>   カメラを動かす場合やウィンドウのサイズを変える際はサイズの更新が行われません．<br>
>   これらの問題はいずれ機能の実装により改善する見込みです.<br>
>   そして,グリッドの表示は2Dカメラのみになっており,今後3Dカメラへの対応も計画されています.<br>

## 今後対応する予定の機能

> ・2Dカメラの追従による動的な座標変更グリッドの実装<br>
> ・2DカメラのZ軸補正<br>
> ・ウィンドウサイズ変更での動的サイズ変更<br>
> ・3Dカメラへの対応<br>