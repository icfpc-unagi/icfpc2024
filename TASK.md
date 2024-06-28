# タスク

多くの人が覚えているように、2006年に私たちは古い社会である「束縛変数のカルト」の文書を発見しました。ICFPコミュニティの貴重な助けを借りて、カルトのコンピューティングデバイス（ユニバーサルマシン）が復活し、多くの興味深い情報を回収することができました。ペゴフカ天文台での最近の観測で、驚くべきことを発見しました。束縛変数のカルトはまだ存在しており、彼らの文明を宇宙に移住させたのです！エンドウが地球から再び脱出した後、彼らの移動を手助けしたのではないかと疑われています。

As many will remember, in 2006 we discovered documents of an old society, called the Cult of the Bound Variable. With the valuable help from the ICFP community, the Cult's computing device (Universal Machine) was brought back to life, and much interesting information could be recovered. In recent observations from the Pegovka observatory, we discovered something stunning: the Cult of the Bound Variable still exists, and has migrated their civilization to space! It is suspected that Endo helped them move, after he escaped from earth again.

数ヶ月の研究の後、私たちは受信したメッセージのほとんどを解読し、通信チャンネルを設定することができました。束縛変数のカルトの人々は現在、星間通信関数型プログラム（ICFP）を使用してコミュニケーションを取っています。ICFP式に関する私たちの発見は、このページに記載されています。

After a couple of months of research, we have been able to decipher most of the received messages, and set up a communication channel. People of the Cult of the Bound variable now use Interstellar Communication Functional Programs (ICFP) to communicate. Our findings about ICFP expressions can be found on this page.

# チャレンジ

通信言語の解読には成功しましたが、残念ながら私たち自身では解決できない課題に直面し、再びみなさまの助けを求めています！通信チャンネルは、束縛変数のカルトのスクールとコミュニケーションを取っています。このスクールは、宇宙での生活に必要なさまざまなスキルを学ぶためのMOOCです。各コースでは一連のテストが出題され、ある基準に基づいて採点されます。

While we were successful in deciphering the communication language, we unfortunately faced challenges that we cannot solve ourselves, and we once again ask for your help! The communication channel communicates with the School of the Bound Variable, a MOOC where students can follow several courses to learn about various skills necessary for life in space. Each of the courses poses a set of tests, which are scored according to some metric.

重要な注意点として、基本的な通信では文字列のみが使用されているようです。そのため、より高度なICFP式は必要になったときに実装し、まずは文字列から始めることをお勧めします。

An important note: it seems that basic communication utilizes strings only. Therefore, we advice you to implement more advanced ICFP expressions only once you need them, and start with strings.

# 通信チャンネル

みなさまに通信を試していただくために、カルトとの通信チャンネルを開放しました。ICFPをボディに含むHTTP POSTリクエストを https://boundvariable.space/communicate に送信すると、リクエストが銀河に送信され、レスポンスが返ってきます。識別のために、チームページにある Authorization ヘッダーを送信する必要があります。

For you to try out the communication, we have opened up the communication channel with the Cult for you. By sending a HTTP POST request to https://boundvariable.space/communicate, with the ICFP in the body, your request is sent into the galaxy and the response is returned to you. For identification purposes, you must send the Authorization header that you can find on your team page.

プログラミング言語から直接POSTリクエストを送信することを強くお勧めしますが、ウェブベースの通信も提供しています。そのページでは、通信履歴も表示されます。

We strongly advise that you make the POST request directly from your favorite programming language, but we also provide web based communication. That page also shows your communication history.

さらに、`S'%4}).$%8` を送信することが、束縛変数のカルトのスクールとコミュニケーションを取るための優れたエントリーポイントであることがわかりました。

Furthermore, we found out that sending `S'%4}).$%8` is a great entrypoint for communicating with the school of the Cult of the Bound Variable.


# 制限

宇宙との通信はエネルギーを消費するプロセスであるため、環境面および金銭面の理由から、メッセージに制限を設けています。リクエストのPOSTボディは1MB（1048576バイト）を超えてはならず、1分間に最大20メッセージまで送信できます。

Communicating with space is an energy consuming process, so for environmental and monetary reasons, we have put limits on the messages. The POST body of your request must not exceed 1 MB (1048576 bytes), and you may send at most 20 messages per minute.


# 採点

各コースのテストは、コースに入ったときに説明される基準に従って採点されます。各テストの最高スコアが保存されます。スコアボードページでは、各コースでのすべてのチームのランクの概要とグローバルランクが表示されます。各コースの個々のテストのすべてのスコアを使用してそのコースのランクを見つけ、コースのランクを使用してグローバルランクを見つけるために、ボルダカウントと呼ばれる方法を使用します。

The tests for each course are scored according to some criterion explained to you when you enter the course. Your best score for each test is stored. On the scoreboard page we've rendered an overview of the ranks of all teams on each course as well as a global rank. To find the ranks for a course given all the scores of the individual tests of that course, and to find the global rank based on the ranks for the courses, we use the so called Borda count.

技術的な観点から言えば、ランクリストは選挙のようなもので、チームが候補者、テストが有権者です。個々のテストでより良い成績を収めるほど、そのテストでのランクが上がります。より直感的な説明としては、各テストで得点する点数は、そのテストであなたよりも厳密に低いスコアを獲得したチームの数であり、そのコースのランクリストはそれらの点数の合計に基づいています。この方法は、まずコースごとのランクリストを計算するために使用され、次に個々のコースのランクを使用してグローバルランクを見つけるために再度使用されます。

In technical terms, the ranklist is an election, where teams are the candidates and the tests are the voters. The better you do for an individual tests, the higher this test will rank you. A more intuitive explanation is that the amount of points you score for each test, is the amount of teams scoring strictly worse than you on that test, and then the ranklist for that course is based on the sum of those points. This method is first used to compute a ranklist per course, and then again using the ranks for the individual courses to find the global rank.

これは抽象的に聞こえるかもしれませんが、知っておくべきことはこれだけだと思います。この評価システムの重要な特性は、絶対的なスコアは重要ではなく、順番だけが重要だということです。また、同点（一部のテストは単に正解/不正解であり、そこではすべての解答者が1位で同点になります）も自然に処理され、異なる絶対スコア範囲のテストが自動的にバランスが取られます。そしてもちろん、すべてのテストで最高のスコアを取ることを目指すべきです！

While this may sound abstract, we believe this is all you need to know. And important property of this rating system is that absolute scores don't matter, only the order. It also deals naturally with ties (some tests are just correct/incorrect, there everyone solving it is tied at 1st place), and automatically balances tests with different absolute score ranges. And of course you should just try get the best scores on all tests!


# 最終コード提出

賞の対象となるには、コンテスト終了間際にチームページからコードを提出してください。コード提出はコンテスト終了の3時間後に締め切られます。ライトニングラウンド用に別途提出する必要はありませんが、最初の24時間のどの部分かを示すREADMEファイルを含めてください。

To be considered for prizes, please submit your code near the end of the contest via your team page. The code submission closes 3 hours after the end of the contest. It is not necessary to make a separate submission for the lightning round, but please include a README file indicating which parts are from the first 24 hours.


# メタ注記

はじめに、様々なICFPプログラミングコンテストについて触れましたが、今年のタスクは完全に新しいものであり、以前のチャレンジの知識は必要ないことを明確にしておきます。しかし、それらのコンテストを楽しんだので、それらに言及しました。今年のコンテストが終了したら、まだ挑戦していない人はぜひ挑戦してみてください！

While the introduction refers to various editions of the ICFP programming contest, we want to make explicit that the tasks set for this year are completely new, and knowledge of the earlier challenges is not necessary. However, we referred to them because we enjoyed those contests, so once this year's contest is over we advice everyone to give them a go in case you haven't yet!

すべてのタイムゾーンでコンテストを公平にするため、コンテスト中にタスクを変更する予定はありません。ただし、ライトニングラウンドの終了時に追加情報を公開する可能性があります。

To make the contest fair for all timezones, we do not intend to make any changes to the task during the contest. However, we might publish some extra information at the end of the lightning round.

そして、参加者への最後のお願いです。コンテストを楽しんでください。しかし、他の人も楽しめるようにしてください！私たちのサーバーを壊そうとしないでください。かなり堅牢なはずですが、ICFPCオーガナイザーはボランティアであり、空き時間にこのコンテストを運営していることを覚えておいてください。そして、これまでの準備の中で、私たち自身は非常に楽しんできました。その楽しさをみなさんと可能な限り共有できればと思います！

And as last request to contestants: enjoy the contest, but also make sure others can enjoy it! Please do not make attempts to break our server, it should be quite robust, but remember that the ICFPC organizers are volunteers that organize this contest in their free time. And so far we tremendously enjoyed ourselves in the preparations, so we hope to share as much as possible of that with you!
